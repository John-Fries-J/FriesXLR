mod mock;

#[cfg(windows)]
mod windows_tusb;

#[cfg(not(windows))]
mod windows_tusb {
    use super::{
        DeviceError, DeviceProvider, DeviceSession, DiscoverySnapshot, DriverInfo,
        SessionGeneration,
    };
    use goxlr_model::DeviceIdentity;

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

        fn open_session(
            &self,
            _identity: &DeviceIdentity,
            _generation: SessionGeneration,
        ) -> Result<Box<dyn DeviceSession>, DeviceError> {
            Err(DeviceError::DriverUnavailable(
                "Windows TUSBAUDIO sessions are not available on this platform".to_string(),
            ))
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

use goxlr_model::{
    ChannelName, CompressorState, DeEsserState, DeviceCapabilities, DeviceIdentity, EqBandId,
    FaderMuteState, FaderName, FaderState, FaderVolume, MicrophoneState, MicrophoneType,
    NoiseGateState, RoutingRoute, RoutingState, VersionNumber,
};
use std::sync::Arc;

pub use mock::MockDeviceProvider;
pub use windows_tusb::WindowsDeviceProvider;

pub type SessionGeneration = u64;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DriverInfo {
    pub interface: Option<String>,
    pub version: Option<VersionNumber>,
    pub available: bool,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceReadOnlyState {
    pub capabilities: DeviceCapabilities,
    pub faders: Vec<FaderState>,
    pub routing: Option<RoutingState>,
    pub microphone: Option<MicrophoneState>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceSessionState {
    pub identity: DeviceIdentity,
    pub capabilities: DeviceCapabilities,
    pub faders: Vec<FaderState>,
    pub routing: Option<RoutingState>,
    pub microphone: Option<MicrophoneState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceEvent {
    FaderVolumeChanged {
        device_id: String,
        generation: SessionGeneration,
        fader: FaderName,
        volume: FaderVolume,
    },
    FaderMuteStateChanged {
        device_id: String,
        generation: SessionGeneration,
        fader: FaderName,
        mute_state: Option<FaderMuteState>,
        muted: Option<bool>,
    },
    FaderMuteButtonChanged {
        device_id: String,
        generation: SessionGeneration,
        fader: FaderName,
        pressed: bool,
    },
    FaderAssignmentChanged {
        device_id: String,
        generation: SessionGeneration,
        fader: FaderName,
        channel: Option<ChannelName>,
    },
    RoutingChanged {
        device_id: String,
        generation: SessionGeneration,
        routing: RoutingState,
    },
    MicrophoneChanged {
        device_id: String,
        generation: SessionGeneration,
        microphone: MicrophoneState,
    },
    Disconnected {
        device_id: String,
        generation: SessionGeneration,
    },
}

impl DeviceEvent {
    pub fn device_id(&self) -> &str {
        match self {
            DeviceEvent::FaderVolumeChanged { device_id, .. }
            | DeviceEvent::FaderMuteStateChanged { device_id, .. }
            | DeviceEvent::FaderMuteButtonChanged { device_id, .. }
            | DeviceEvent::FaderAssignmentChanged { device_id, .. }
            | DeviceEvent::RoutingChanged { device_id, .. }
            | DeviceEvent::MicrophoneChanged { device_id, .. }
            | DeviceEvent::Disconnected { device_id, .. } => device_id,
        }
    }

    pub fn generation(&self) -> SessionGeneration {
        match self {
            DeviceEvent::FaderVolumeChanged { generation, .. }
            | DeviceEvent::FaderMuteStateChanged { generation, .. }
            | DeviceEvent::FaderMuteButtonChanged { generation, .. }
            | DeviceEvent::FaderAssignmentChanged { generation, .. }
            | DeviceEvent::RoutingChanged { generation, .. }
            | DeviceEvent::MicrophoneChanged { generation, .. }
            | DeviceEvent::Disconnected { generation, .. } => *generation,
        }
    }
}

pub trait DeviceProvider: Send + Sync {
    fn discover(&self) -> Result<DiscoverySnapshot, DeviceError>;

    fn open_session(
        &self,
        identity: &DeviceIdentity,
        generation: SessionGeneration,
    ) -> Result<Box<dyn DeviceSession>, DeviceError>;

    fn driver_info(&self) -> DriverInfo;
}

pub trait DeviceSession: Send {
    fn generation(&self) -> SessionGeneration;
    fn current_state(&self) -> Result<DeviceSessionState, DeviceError>;
    fn poll_event(&mut self) -> Result<Option<DeviceEvent>, DeviceError>;
    fn close(&mut self);

    fn set_fader_volume(
        &mut self,
        _fader: FaderName,
        _volume: FaderVolume,
    ) -> Result<(), DeviceError> {
        Err(DeviceError::UnsupportedOperation(
            "fader volume writes are not enabled for this device".to_string(),
        ))
    }

    fn set_fader_mute(&mut self, _fader: FaderName, _muted: bool) -> Result<(), DeviceError> {
        Err(DeviceError::UnsupportedOperation(
            "fader mute writes are not enabled for this device".to_string(),
        ))
    }

    fn set_fader_assignment(
        &mut self,
        _fader: FaderName,
        _channel: ChannelName,
    ) -> Result<(), DeviceError> {
        Err(DeviceError::UnsupportedOperation(
            "fader assignment writes are not enabled for this device".to_string(),
        ))
    }

    fn set_routing_route(
        &mut self,
        _route: RoutingRoute,
        _enabled: bool,
    ) -> Result<(), DeviceError> {
        Err(DeviceError::UnsupportedOperation(
            "routing writes are not enabled for this device".to_string(),
        ))
    }

    fn set_microphone_type(
        &mut self,
        _microphone_type: MicrophoneType,
        _confirm_phantom_power: bool,
    ) -> Result<(), DeviceError> {
        Err(DeviceError::UnsupportedOperation(
            "microphone type writes are not enabled for this device".to_string(),
        ))
    }

    fn set_microphone_gain(
        &mut self,
        _microphone_type: MicrophoneType,
        _gain_db: u16,
    ) -> Result<(), DeviceError> {
        Err(DeviceError::UnsupportedOperation(
            "microphone gain writes are not enabled for this device".to_string(),
        ))
    }

    fn set_equalizer_band(
        &mut self,
        _band_id: EqBandId,
        _frequency_tenths_hz: u32,
        _gain_db: i8,
    ) -> Result<(), DeviceError> {
        Err(DeviceError::UnsupportedOperation(
            "equalizer writes are not enabled for this device".to_string(),
        ))
    }

    fn set_noise_gate(&mut self, _gate: NoiseGateState) -> Result<(), DeviceError> {
        Err(DeviceError::UnsupportedOperation(
            "noise gate writes are not enabled for this device".to_string(),
        ))
    }

    fn set_compressor(&mut self, _compressor: CompressorState) -> Result<(), DeviceError> {
        Err(DeviceError::UnsupportedOperation(
            "compressor writes are not enabled for this device".to_string(),
        ))
    }

    fn set_de_esser(&mut self, _de_esser: DeEsserState) -> Result<(), DeviceError> {
        Err(DeviceError::UnsupportedOperation(
            "de-esser writes are not enabled for this device".to_string(),
        ))
    }
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
        let mut snapshot = match self.hardware.discover() {
            Ok(snapshot) => snapshot,
            Err(DeviceError::DriverUnavailable(error)) => {
                tracing::debug!(%error, "GoXLR hardware driver unavailable during discovery");
                DiscoverySnapshot::default()
            }
            Err(error) => return Err(error),
        };
        snapshot.devices.extend(self.mock.discover()?.devices);
        Ok(snapshot)
    }

    fn open_session(
        &self,
        identity: &DeviceIdentity,
        generation: SessionGeneration,
    ) -> Result<Box<dyn DeviceSession>, DeviceError> {
        if identity.is_mock {
            return self.mock.open_session(identity, generation);
        }

        self.hardware.open_session(identity, generation)
    }

    fn driver_info(&self) -> DriverInfo {
        self.hardware.driver_info()
    }
}

pub fn unavailable_faders() -> Vec<FaderState> {
    FaderName::ALL
        .into_iter()
        .map(|name| FaderState {
            name,
            assigned_channel: None,
            volume: None,
            mute_state: None,
            mute_function: None,
            muted: None,
            mute_button_pressed: None,
        })
        .collect()
}

#[derive(Debug, thiserror::Error)]
pub enum DeviceError {
    #[error("Windows GoXLR driver API is not available: {0}")]
    DriverUnavailable(String),

    #[error("device discovery failed: {0}")]
    Discovery(String),

    #[error("device is unavailable: {0}")]
    DeviceUnavailable(String),

    #[error("device disconnected")]
    DeviceDisconnected,

    #[error("device session is closed")]
    SessionClosed,

    #[error("protocol error: {0}")]
    Protocol(String),

    #[error("request timed out: {0}")]
    Timeout(String),

    #[error("unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("malformed response: {0}")]
    MalformedResponse(String),

    #[error("stale session")]
    StaleSession,
}

impl From<goxlr_protocol::ProtocolError> for DeviceError {
    fn from(value: goxlr_protocol::ProtocolError) -> Self {
        match value {
            goxlr_protocol::ProtocolError::MalformedResponse { reason } => {
                DeviceError::MalformedResponse(reason)
            }
            other => DeviceError::Protocol(other.to_string()),
        }
    }
}

impl From<goxlr_model::ModelError> for DeviceError {
    fn from(value: goxlr_model::ModelError) -> Self {
        DeviceError::UnsupportedOperation(value.to_string())
    }
}
