use goxlr_device::{
    CompositeDeviceProvider, DeviceConnectionInfo, DeviceProvider, MockDeviceProvider,
};
use goxlr_model::{
    AppSnapshot, ConnectionStatus, DeviceState, FaderName, FaderState, ServiceStatus,
};
use goxlr_profile::{AppConfig, ConfigError, ConfigStore};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{broadcast, Notify, RwLock};
use tracing::{debug, info, warn};

const DISCOVERY_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Debug, Clone)]
pub enum ServiceEvent {
    SnapshotChanged(AppSnapshot),
}

pub struct AppService {
    config_store: ConfigStore,
    provider: CompositeDeviceProvider,
    snapshot: RwLock<AppSnapshot>,
    events: broadcast::Sender<ServiceEvent>,
    shutdown_requested: AtomicBool,
    shutdown: Notify,
}

impl AppService {
    pub async fn new(config_store: ConfigStore) -> Result<Arc<Self>, ServiceError> {
        let config = config_store.load_or_default()?;
        let mock = Arc::new(MockDeviceProvider::new(config.mock_device_enabled));
        let provider = CompositeDeviceProvider::new(mock);
        let snapshot = AppSnapshot::disconnected(settings_summary(&config_store, &config));
        let (events, _) = broadcast::channel(32);

        Ok(Arc::new(Self {
            config_store,
            provider,
            snapshot: RwLock::new(snapshot),
            events,
            shutdown_requested: AtomicBool::new(false),
            shutdown: Notify::new(),
        }))
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ServiceEvent> {
        self.events.subscribe()
    }

    pub async fn snapshot(&self) -> AppSnapshot {
        self.snapshot.read().await.clone()
    }

    pub async fn set_mock_device_enabled(
        &self,
        enabled: bool,
    ) -> Result<AppSnapshot, ServiceError> {
        let mut config = self.config_store.load_or_default()?;
        if config.mock_device_enabled != enabled {
            config.mock_device_enabled = enabled;
            self.config_store.save(&config)?;
        }

        self.provider.set_mock_enabled(enabled);
        self.refresh_once().await?;
        Ok(self.snapshot().await)
    }

    pub async fn set_start_minimized(&self, enabled: bool) -> Result<AppSnapshot, ServiceError> {
        let mut config = self.config_store.load_or_default()?;
        if config.start_minimized != enabled {
            config.start_minimized = enabled;
            self.config_store.save(&config)?;
        }

        self.refresh_once().await?;
        Ok(self.snapshot().await)
    }

    pub async fn set_launch_at_startup(&self, enabled: bool) -> Result<AppSnapshot, ServiceError> {
        let mut config = self.config_store.load_or_default()?;
        if config.launch_at_startup != enabled {
            config.launch_at_startup = enabled;
            self.config_store.save(&config)?;
        }

        self.refresh_once().await?;
        Ok(self.snapshot().await)
    }

    pub fn request_shutdown(&self) {
        self.shutdown_requested.store(true, Ordering::SeqCst);
        self.shutdown.notify_waiters();
    }

    pub async fn shutdown(&self) {
        self.request_shutdown();

        let mut snapshot = self.snapshot.read().await.clone();
        snapshot.service.running = false;
        self.replace_snapshot(snapshot).await;
    }

    pub async fn run(self: Arc<Self>) {
        info!("starting FriesXLR service loop");

        while !self.shutdown_requested.load(Ordering::SeqCst) {
            if let Err(error) = self.refresh_once().await {
                warn!(%error, "device refresh failed");
                self.set_service_error(error.to_string()).await;
            }

            if self.shutdown_requested.load(Ordering::SeqCst) {
                break;
            }

            tokio::select! {
                _ = tokio::time::sleep(DISCOVERY_INTERVAL) => {}
                _ = self.shutdown.notified() => {}
            }
        }

        info!("FriesXLR service loop stopped");
    }

    pub async fn refresh_once(&self) -> Result<(), ServiceError> {
        let config = self.config_store.load_or_default()?;
        self.provider.set_mock_enabled(config.mock_device_enabled);

        let discovery = self.provider.discover()?;
        let devices = discovery
            .devices
            .into_iter()
            .map(device_state_from_connection)
            .collect::<Vec<_>>();

        let mut next = AppSnapshot {
            settings: settings_summary(&self.config_store, &config),
            service: ServiceStatus {
                running: !self.shutdown_requested.load(Ordering::SeqCst),
                last_error: None,
            },
            devices,
        };

        let driver = self.provider.driver_info();
        if !driver.available {
            next.service.last_error = driver.last_error;
        }

        self.replace_snapshot(next).await;
        Ok(())
    }

    async fn set_service_error(&self, error: String) {
        let mut snapshot = self.snapshot.read().await.clone();
        snapshot.service.last_error = Some(error);
        self.replace_snapshot(snapshot).await;
    }

    async fn replace_snapshot(&self, next: AppSnapshot) {
        let mut current = self.snapshot.write().await;
        if *current == next {
            debug!("snapshot unchanged");
            return;
        }

        *current = next.clone();
        if let Err(error) = self.events.send(ServiceEvent::SnapshotChanged(next)) {
            debug!(%error, "no active event subscribers");
        }
    }
}

fn settings_summary(
    config_store: &ConfigStore,
    config: &AppConfig,
) -> goxlr_model::AppSettingsSummary {
    config.summary(Some(config_store.path().display().to_string()))
}

fn device_state_from_connection(connection: DeviceConnectionInfo) -> DeviceState {
    let faders = connection
        .read_only_state
        .map(|state| state.faders)
        .unwrap_or_else(empty_faders);

    DeviceState {
        identity: connection.identity,
        status: ConnectionStatus::Connected,
        faders,
        last_seen_epoch_ms: epoch_ms(),
    }
}

fn empty_faders() -> Vec<FaderState> {
    [FaderName::A, FaderName::B, FaderName::C, FaderName::D]
        .into_iter()
        .map(|name| FaderState {
            name,
            assigned_channel: None,
            volume: None,
            muted: None,
        })
        .collect()
}

fn epoch_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Device(#[from] goxlr_device::DeviceError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn enabling_mock_device_updates_snapshot() {
        let dir = tempfile::tempdir().unwrap();
        let store = ConfigStore::new(dir.path().join("config.json"));
        let service = AppService::new(store).await.unwrap();

        let snapshot = service.set_mock_device_enabled(true).await.unwrap();

        let mock_devices = snapshot
            .devices
            .iter()
            .filter(|device| device.identity.is_mock)
            .count();

        assert_eq!(mock_devices, 1);
    }

    #[test]
    fn creates_empty_fader_state_for_real_discovery_without_control_reads() {
        let faders = empty_faders();

        assert_eq!(faders.len(), 4);
        assert!(faders.iter().all(|fader| fader.volume.is_none()));
    }

    #[tokio::test]
    async fn service_stops_when_shutdown_is_requested() {
        let dir = tempfile::tempdir().unwrap();
        let store = ConfigStore::new(dir.path().join("config.json"));
        let service = AppService::new(store).await.unwrap();
        let runner = service.clone();

        let handle = tokio::spawn(async move {
            runner.run().await;
        });

        service.request_shutdown();

        tokio::time::timeout(Duration::from_secs(1), handle)
            .await
            .unwrap()
            .unwrap();
    }
}
