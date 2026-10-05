use goxlr_device::{
    CompositeDeviceProvider, DeviceConnectionInfo, DeviceError, DeviceEvent, DeviceProvider,
    DeviceSession, DeviceSessionState, MockDeviceProvider, SessionGeneration,
};
use goxlr_model::{
    AppSnapshot, ChannelName, ConnectionStatus, DeviceState, FaderName, FaderState, FaderVolume,
    ServiceStatus,
};
use goxlr_profile::{AppConfig, ConfigError, ConfigStore};
use std::collections::{HashMap, HashSet};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::{broadcast, Mutex, Notify, RwLock};
use tracing::{debug, info, warn};

const DISCOVERY_INTERVAL: Duration = Duration::from_secs(1);
const SESSION_POLL_INTERVAL: Duration = Duration::from_millis(50);

#[derive(Debug, Clone)]
pub enum ServiceEvent {
    SnapshotChanged(AppSnapshot),
}

pub struct AppService {
    config_store: ConfigStore,
    provider: CompositeDeviceProvider,
    runtime: Mutex<ServiceRuntime>,
    snapshot: RwLock<AppSnapshot>,
    events: broadcast::Sender<ServiceEvent>,
    shutdown_requested: AtomicBool,
    shutdown: Notify,
}

#[derive(Default)]
struct ServiceRuntime {
    sessions: HashMap<String, ActiveSession>,
    selected_device_id: Option<String>,
    next_generation: SessionGeneration,
}

struct ActiveSession {
    session: Box<dyn DeviceSession>,
    state: DeviceSessionState,
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
            runtime: Mutex::new(ServiceRuntime::default()),
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

    pub async fn set_selected_device(
        &self,
        device_id: Option<String>,
    ) -> Result<AppSnapshot, ServiceError> {
        let config = self.config_store.load_or_default()?;
        {
            let mut runtime = self.runtime.lock().await;
            if let Some(device_id) = &device_id {
                if !runtime.sessions.contains_key(device_id) {
                    return Err(ServiceError::Device(DeviceError::DeviceUnavailable(
                        device_id.clone(),
                    )));
                }
            }
            runtime.selected_device_id = device_id;
        }

        self.publish_snapshot(&config).await;
        Ok(self.snapshot().await)
    }

    pub async fn set_fader_volume(
        &self,
        device_id: String,
        session_generation: SessionGeneration,
        fader: FaderName,
        percent: u8,
    ) -> Result<AppSnapshot, ServiceError> {
        let volume = FaderVolume::from_percent(percent)?;
        self.with_session_mut(&device_id, session_generation, |session| {
            session.set_fader_volume(fader, volume)
        })
        .await?;
        self.refresh_session_state(&device_id).await?;

        Ok(self.snapshot().await)
    }

    pub async fn set_fader_mute(
        &self,
        device_id: String,
        session_generation: SessionGeneration,
        fader: FaderName,
        muted: bool,
    ) -> Result<AppSnapshot, ServiceError> {
        self.with_session_mut(&device_id, session_generation, |session| {
            session.set_fader_mute(fader, muted)
        })
        .await?;
        self.refresh_session_state(&device_id).await?;

        Ok(self.snapshot().await)
    }

    pub async fn set_fader_assignment(
        &self,
        device_id: String,
        session_generation: SessionGeneration,
        fader: FaderName,
        channel: ChannelName,
    ) -> Result<AppSnapshot, ServiceError> {
        self.with_session_mut(&device_id, session_generation, |session| {
            session.set_fader_assignment(fader, channel)
        })
        .await?;
        self.refresh_session_state(&device_id).await?;

        Ok(self.snapshot().await)
    }

    pub fn request_shutdown(&self) {
        self.shutdown_requested.store(true, Ordering::SeqCst);
        self.shutdown.notify_waiters();
    }

    pub async fn shutdown(&self) {
        self.request_shutdown();

        {
            let mut runtime = self.runtime.lock().await;
            for active in runtime.sessions.values_mut() {
                active.session.close();
            }
            runtime.sessions.clear();
        }

        let mut snapshot = self.snapshot.read().await.clone();
        snapshot.service.running = false;
        self.replace_snapshot(snapshot).await;
    }

    pub async fn run(self: Arc<Self>) {
        info!("starting FriesXLR service loop");
        let mut next_discovery = Instant::now();

        while !self.shutdown_requested.load(Ordering::SeqCst) {
            let result = if Instant::now() >= next_discovery {
                next_discovery = Instant::now() + DISCOVERY_INTERVAL;
                self.refresh_once().await
            } else {
                self.poll_sessions_once().await
            };

            if let Err(error) = result {
                warn!(%error, "service update failed");
                self.set_service_error(error.to_string()).await;
            }

            if self.shutdown_requested.load(Ordering::SeqCst) {
                break;
            }

            tokio::select! {
                _ = tokio::time::sleep(SESSION_POLL_INTERVAL) => {}
                _ = self.shutdown.notified() => {}
            }
        }

        info!("FriesXLR service loop stopped");
    }

    pub async fn refresh_once(&self) -> Result<(), ServiceError> {
        let config = self.config_store.load_or_default()?;
        self.provider.set_mock_enabled(config.mock_device_enabled);

        let discovery = self.provider.discover()?;
        self.reconcile_discovery(discovery.devices).await;
        self.poll_sessions_locked().await?;
        self.publish_snapshot(&config).await;
        Ok(())
    }

    async fn poll_sessions_once(&self) -> Result<(), ServiceError> {
        let config = self.config_store.load_or_default()?;
        self.poll_sessions_locked().await?;
        self.publish_snapshot(&config).await;
        Ok(())
    }

    async fn reconcile_discovery(&self, devices: Vec<DeviceConnectionInfo>) {
        let discovered_ids = devices
            .iter()
            .map(|device| device.identity.id.clone())
            .collect::<HashSet<_>>();

        let mut runtime = self.runtime.lock().await;
        let stale_ids = runtime
            .sessions
            .keys()
            .filter(|device_id| !discovered_ids.contains(*device_id))
            .cloned()
            .collect::<Vec<_>>();

        for device_id in stale_ids {
            if let Some(mut active) = runtime.sessions.remove(&device_id) {
                active.session.close();
                info!(%device_id, "device disconnected");
            }
        }

        for connection in devices {
            if runtime.sessions.contains_key(&connection.identity.id) {
                continue;
            }

            runtime.next_generation = runtime.next_generation.saturating_add(1);
            let generation = runtime.next_generation;
            let device_id = connection.identity.id.clone();
            match self.provider.open_session(&connection.identity, generation) {
                Ok(session) => match session.current_state() {
                    Ok(state) => {
                        info!(
                            %device_id,
                            session_generation = generation,
                            "created device session"
                        );
                        runtime
                            .sessions
                            .insert(device_id.clone(), ActiveSession { session, state });
                    }
                    Err(error) => {
                        warn!(%device_id, %error, "opened session but could not read state");
                    }
                },
                Err(error) => {
                    warn!(%device_id, %error, "failed to open device session");
                }
            }
        }

        ensure_selected_device(&mut runtime);
    }

    async fn poll_sessions_locked(&self) -> Result<(), ServiceError> {
        let mut runtime = self.runtime.lock().await;
        let device_ids = runtime.sessions.keys().cloned().collect::<Vec<_>>();
        let mut disconnected = Vec::new();

        for device_id in device_ids {
            let Some(active) = runtime.sessions.get_mut(&device_id) else {
                continue;
            };

            loop {
                let event = active.session.poll_event()?;
                let Some(event) = event else {
                    break;
                };

                if event.generation() != active.session.generation() {
                    warn!(
                        device_id = event.device_id(),
                        event_generation = event.generation(),
                        session_generation = active.session.generation(),
                        "ignored stale device event"
                    );
                    continue;
                }

                if matches!(event, DeviceEvent::Disconnected { .. }) {
                    disconnected.push(device_id.clone());
                    break;
                }

                apply_event_to_state(&mut active.state, event);
            }
        }

        for device_id in disconnected {
            if let Some(mut active) = runtime.sessions.remove(&device_id) {
                active.session.close();
                info!(%device_id, "device session removed after disconnect");
            }
        }

        ensure_selected_device(&mut runtime);
        Ok(())
    }

    async fn refresh_session_state(&self, device_id: &str) -> Result<(), ServiceError> {
        let config = self.config_store.load_or_default()?;
        {
            let mut runtime = self.runtime.lock().await;
            let active = runtime
                .sessions
                .get_mut(device_id)
                .ok_or_else(|| DeviceError::DeviceUnavailable(device_id.to_string()))?;
            active.state = active.session.current_state()?;
        }
        self.publish_snapshot(&config).await;
        Ok(())
    }

    async fn with_session_mut<F>(
        &self,
        device_id: &str,
        session_generation: SessionGeneration,
        operation: F,
    ) -> Result<(), ServiceError>
    where
        F: FnOnce(&mut dyn DeviceSession) -> Result<(), DeviceError>,
    {
        let mut runtime = self.runtime.lock().await;
        let active = runtime
            .sessions
            .get_mut(device_id)
            .ok_or_else(|| DeviceError::DeviceUnavailable(device_id.to_string()))?;

        if active.session.generation() != session_generation {
            return Err(ServiceError::Device(DeviceError::StaleSession));
        }

        operation(active.session.as_mut())?;
        Ok(())
    }

    async fn publish_snapshot(&self, config: &AppConfig) {
        let mut next = {
            let runtime = self.runtime.lock().await;
            AppSnapshot {
                settings: settings_summary(&self.config_store, config),
                service: ServiceStatus {
                    running: !self.shutdown_requested.load(Ordering::SeqCst),
                    last_error: None,
                },
                devices: runtime
                    .sessions
                    .values()
                    .map(device_state_from_session)
                    .collect(),
                selected_device_id: runtime.selected_device_id.clone(),
            }
        };

        let driver = self.provider.driver_info();
        if !driver.available {
            next.service.last_error = driver.last_error;
        }

        self.replace_snapshot(next).await;
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

fn ensure_selected_device(runtime: &mut ServiceRuntime) {
    let selected_valid = runtime
        .selected_device_id
        .as_ref()
        .map(|device_id| runtime.sessions.contains_key(device_id))
        .unwrap_or(false);

    if !selected_valid {
        runtime.selected_device_id = runtime.sessions.keys().next().cloned();
    }
}

fn device_state_from_session(active: &ActiveSession) -> DeviceState {
    DeviceState {
        identity: active.state.identity.clone(),
        status: ConnectionStatus::Connected,
        capabilities: active.state.capabilities.clone(),
        faders: active.state.faders.clone(),
        last_seen_epoch_ms: epoch_ms(),
        session_generation: Some(active.session.generation()),
    }
}

fn apply_event_to_state(state: &mut DeviceSessionState, event: DeviceEvent) {
    match event {
        DeviceEvent::FaderVolumeChanged { fader, volume, .. } => {
            if let Some(fader_state) = fader_mut(&mut state.faders, fader) {
                fader_state.volume = Some(volume);
            }
        }
        DeviceEvent::FaderMuteStateChanged {
            fader,
            mute_state,
            muted,
            ..
        } => {
            if let Some(fader_state) = fader_mut(&mut state.faders, fader) {
                fader_state.mute_state = mute_state;
                fader_state.muted = muted;
            }
        }
        DeviceEvent::FaderMuteButtonChanged { fader, pressed, .. } => {
            if let Some(fader_state) = fader_mut(&mut state.faders, fader) {
                fader_state.mute_button_pressed = Some(pressed);
            }
        }
        DeviceEvent::FaderAssignmentChanged { fader, channel, .. } => {
            if let Some(fader_state) = fader_mut(&mut state.faders, fader) {
                fader_state.assigned_channel = channel;
            }
        }
        DeviceEvent::Disconnected { .. } => {}
    }
}

fn fader_mut(faders: &mut [FaderState], fader: FaderName) -> Option<&mut FaderState> {
    faders.iter_mut().find(|state| state.name == fader)
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

    #[error(transparent)]
    Model(#[from] goxlr_model::ModelError),
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
        assert_eq!(
            snapshot.selected_device_id.as_deref(),
            Some("mock:goxlr-mini:dev")
        );
    }

    #[tokio::test]
    async fn mock_volume_command_updates_authoritative_snapshot() {
        let dir = tempfile::tempdir().unwrap();
        let store = ConfigStore::new(dir.path().join("config.json"));
        let service = AppService::new(store).await.unwrap();
        let snapshot = service.set_mock_device_enabled(true).await.unwrap();
        let device = &snapshot.devices[0];

        let snapshot = service
            .set_fader_volume(
                device.identity.id.clone(),
                device.session_generation.unwrap(),
                FaderName::A,
                25,
            )
            .await
            .unwrap();

        assert_eq!(snapshot.devices[0].faders[0].volume.unwrap().percent, 25);
    }

    #[tokio::test]
    async fn stale_session_generation_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let store = ConfigStore::new(dir.path().join("config.json"));
        let service = AppService::new(store).await.unwrap();
        let snapshot = service.set_mock_device_enabled(true).await.unwrap();
        let device_id = snapshot.devices[0].identity.id.clone();

        let error = service
            .set_fader_volume(device_id, 999, FaderName::A, 25)
            .await
            .unwrap_err();

        assert!(matches!(
            error,
            ServiceError::Device(DeviceError::StaleSession)
        ));
    }

    #[tokio::test]
    async fn selected_device_can_be_changed() {
        let dir = tempfile::tempdir().unwrap();
        let store = ConfigStore::new(dir.path().join("config.json"));
        let service = AppService::new(store).await.unwrap();
        service.set_mock_device_enabled(true).await.unwrap();

        let snapshot = service
            .set_selected_device(Some("mock:goxlr-mini:dev".to_string()))
            .await
            .unwrap();

        assert_eq!(
            snapshot.selected_device_id.as_deref(),
            Some("mock:goxlr-mini:dev")
        );
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
