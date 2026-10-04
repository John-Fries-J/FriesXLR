mod mock;

#[cfg(windows)]
mod windows_tusb;

#[cfg(not(windows))]
mod windows_tusb {
    use super::{DeviceError, DeviceProvider, DiscoverySnapshot, DriverInfo};

    #[derive(Debug, Default)]
    pub struct WindowsDeviceProvider;

    impl WindowsDeviceProvider {
        pub fn new() -> Self {
            Self
        }
    }

    impl DeviceProvider for WindowsDeviceProvider {
        fn discover(&self) -> Result<DiscoverySnapshot, DeviceError> {
            Ok(DiscoverySnapshot::default())
        }

        fn driver_info(&self) -> DriverInfo {
            DriverInfo {
                interface: Some("unsupported-on-this-platform".to_string()),
                version: None,
                available: false,
                last_error: None,
            }
        }
    }
}

use goxlr_model::{DeviceIdentity, FaderState, VersionNumber};
use std::sync::Arc;

pub use mock::MockDeviceProvider;
pub use windows_tusb::WindowsDeviceProvider;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DriverInfo {
    pub interface: Option<String>,
    pub version: Option<VersionNumber>,
    pub available: bool,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceReadOnlyState {
    pub faders: Vec<FaderState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceConnectionInfo {
    pub identity: DeviceIdentity,
    pub read_only_state: Option<DeviceReadOnlyState>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DiscoverySnapshot {
    pub devices: Vec<DeviceConnectionInfo>,
}

pub trait DeviceProvider: Send + Sync {
    fn discover(&self) -> Result<DiscoverySnapshot, DeviceError>;
    fn driver_info(&self) -> DriverInfo;
}

pub struct CompositeDeviceProvider {
    hardware: WindowsDeviceProvider,
    mock: Arc<MockDeviceProvider>,
}

impl CompositeDeviceProvider {
    pub fn new(mock: Arc<MockDeviceProvider>) -> Self {
        Self {
            hardware: WindowsDeviceProvider::new(),
            mock,
        }
    }

    pub fn set_mock_enabled(&self, enabled: bool) {
        self.mock.set_enabled(enabled);
    }
}

impl DeviceProvider for CompositeDeviceProvider {
    fn discover(&self) -> Result<DiscoverySnapshot, DeviceError> {
        let mut snapshot = self.hardware.discover()?;
        snapshot.devices.extend(self.mock.discover()?.devices);
        Ok(snapshot)
    }

    fn driver_info(&self) -> DriverInfo {
        self.hardware.driver_info()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeviceError {
    #[error("Windows GoXLR driver API is not available: {0}")]
    DriverUnavailable(String),

    #[error("device discovery failed: {0}")]
    Discovery(String),
}
