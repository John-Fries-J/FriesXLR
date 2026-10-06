use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeviceModel {
    GoXlr,
    GoXlrMini,
    Unknown,
}

impl Display for DeviceModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceModel::GoXlr => f.write_str("GoXLR"),
            DeviceModel::GoXlrMini => f.write_str("GoXLR Mini"),
            DeviceModel::Unknown => f.write_str("Unknown"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionNumber {
    pub major: u32,
    pub minor: u32,
    pub patch: Option<u32>,
    pub build: Option<u32>,
}

impl Display for VersionNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (self.patch, self.build) {
            (Some(patch), Some(build)) => {
                write!(f, "{}.{}.{}.{}", self.major, self.minor, patch, build)
            }
            (Some(patch), None) => write!(f, "{}.{}.{}", self.major, self.minor, patch),
            (None, _) => write!(f, "{}.{}", self.major, self.minor),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FaderName {
    A,
    B,
    C,
    D,
}

impl FaderName {
    pub const ALL: [Self; 4] = [Self::A, Self::B, Self::C, Self::D];

    pub fn index(self) -> usize {
        match self {
            FaderName::A => 0,
            FaderName::B => 1,
            FaderName::C => 2,
            FaderName::D => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChannelName {
    Mic,
    LineIn,
    Console,
    System,
    Game,
    Chat,
    Sample,
    Music,
    Headphones,
    MicMonitor,
    LineOut,
    Unknown,
}

impl ChannelName {
    pub const ASSIGNABLE: [Self; 10] = [
        Self::Mic,
        Self::LineIn,
        Self::Console,
        Self::System,
        Self::Game,
        Self::Chat,
        Self::Sample,
        Self::Music,
        Self::Headphones,
        Self::LineOut,
    ];
}

impl Display for ChannelName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelName::Mic => f.write_str("Mic"),
            ChannelName::LineIn => f.write_str("Line In"),
            ChannelName::Console => f.write_str("Console"),
            ChannelName::System => f.write_str("System"),
            ChannelName::Game => f.write_str("Game"),
            ChannelName::Chat => f.write_str("Chat"),
            ChannelName::Sample => f.write_str("Sample"),
            ChannelName::Music => f.write_str("Music"),
            ChannelName::Headphones => f.write_str("Headphones"),
            ChannelName::MicMonitor => f.write_str("Mic Monitor"),
            ChannelName::LineOut => f.write_str("Line Out"),
            ChannelName::Unknown => f.write_str("Unknown"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MuteFunction {
    All,
    ToStream,
    ToVoiceChat,
    ToPhones,
    ToLineOut,
    ToStream2,
    ToStreams,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FaderMuteState {
    Unmuted,
    MutedToX,
    MutedToAll,
    Unknown,
}

impl FaderMuteState {
    pub fn is_muted(self) -> Option<bool> {
        match self {
            FaderMuteState::Unmuted => Some(false),
            FaderMuteState::MutedToX | FaderMuteState::MutedToAll => Some(true),
            FaderMuteState::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaderVolume {
    pub raw: u8,
    pub percent: u8,
}

impl FaderVolume {
    pub fn from_raw(raw: u8) -> Self {
        Self {
            raw,
            percent: raw_to_percent(raw),
        }
    }

    pub fn from_percent(percent: u8) -> Result<Self, ModelError> {
        Ok(Self {
            raw: percent_to_raw(percent)?,
            percent,
        })
    }
}

pub fn raw_to_percent(raw: u8) -> u8 {
    (((raw as u16) * 100 + 127) / 255) as u8
}

pub fn percent_to_raw(percent: u8) -> Result<u8, ModelError> {
    if percent > 100 {
        return Err(ModelError::InvalidVolumePercent(percent));
    }

    Ok((((percent as u16) * 255 + 50) / 100) as u8)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCapabilities {
    pub readable_fader_assignments: bool,
    pub readable_fader_volumes: bool,
    pub readable_fader_mute_state: bool,
    pub readable_fader_button_state: bool,
    pub writable_fader_volumes: bool,
    pub writable_fader_mute_state: bool,
    pub writable_fader_assignments: bool,
    pub supported_assignment_channels: Vec<ChannelName>,
}

impl DeviceCapabilities {
    pub fn unavailable() -> Self {
        Self {
            readable_fader_assignments: false,
            readable_fader_volumes: false,
            readable_fader_mute_state: false,
            readable_fader_button_state: false,
            writable_fader_volumes: false,
            writable_fader_mute_state: false,
            writable_fader_assignments: false,
            supported_assignment_channels: Vec::new(),
        }
    }

    pub fn mock() -> Self {
        Self {
            readable_fader_assignments: true,
            readable_fader_volumes: true,
            readable_fader_mute_state: true,
            readable_fader_button_state: true,
            writable_fader_volumes: true,
            writable_fader_mute_state: true,
            writable_fader_assignments: true,
            supported_assignment_channels: ChannelName::ASSIGNABLE.to_vec(),
        }
    }

    pub fn physical_read_only() -> Self {
        Self {
            readable_fader_assignments: false,
            readable_fader_volumes: true,
            readable_fader_mute_state: false,
            readable_fader_button_state: true,
            writable_fader_volumes: false,
            writable_fader_mute_state: false,
            writable_fader_assignments: false,
            supported_assignment_channels: ChannelName::ASSIGNABLE.to_vec(),
        }
    }
}

impl Default for DeviceCapabilities {
    fn default() -> Self {
        Self::unavailable()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceIdentity {
    pub id: String,
    pub model: DeviceModel,
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
    pub manufacturer_name: Option<String>,
    pub product_name: Option<String>,
    pub serial_number: Option<String>,
    pub firmware_version: Option<VersionNumber>,
    pub driver_interface: Option<String>,
    pub driver_version: Option<VersionNumber>,
    pub is_mock: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaderState {
    pub name: FaderName,
    pub assigned_channel: Option<ChannelName>,
    pub volume: Option<FaderVolume>,
    pub mute_state: Option<FaderMuteState>,
    pub mute_function: Option<MuteFunction>,
    pub muted: Option<bool>,
    pub mute_button_pressed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceState {
    pub identity: DeviceIdentity,
    pub status: ConnectionStatus,
    pub capabilities: DeviceCapabilities,
    pub faders: Vec<FaderState>,
    pub last_seen_epoch_ms: u64,
    pub session_generation: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettingsSummary {
    pub mock_device_enabled: bool,
    pub show_tray_icon: bool,
    pub start_minimized: bool,
    pub launch_at_startup: bool,
    pub log_level: LogLevel,
    pub config_path: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Error => f.write_str("error"),
            LogLevel::Warn => f.write_str("warn"),
            LogLevel::Info => f.write_str("info"),
            LogLevel::Debug => f.write_str("debug"),
            LogLevel::Trace => f.write_str("trace"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStatus {
    pub running: bool,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSnapshot {
    pub settings: AppSettingsSummary,
    pub service: ServiceStatus,
    pub devices: Vec<DeviceState>,
    pub selected_device_id: Option<String>,
}

impl AppSnapshot {
    pub fn disconnected(settings: AppSettingsSummary) -> Self {
        Self {
            settings,
            service: ServiceStatus {
                running: true,
                last_error: None,
            },
            devices: Vec::new(),
            selected_device_id: None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("invalid fader volume {0}; expected 0..=255")]
    InvalidFaderVolume(u16),

    #[error("invalid fader volume percentage {0}; expected 0..=100")]
    InvalidVolumePercent(u8),
}

pub fn validate_volume(volume: u16) -> Result<u8, ModelError> {
    u8::try_from(volume).map_err(|_| ModelError::InvalidFaderVolume(volume))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_versions_with_optional_parts() {
        let version = VersionNumber {
            major: 1,
            minor: 3,
            patch: Some(40),
            build: Some(12),
        };

        assert_eq!(version.to_string(), "1.3.40.12");
    }

    #[test]
    fn rejects_volume_outside_device_range() {
        assert!(validate_volume(256).is_err());
        assert_eq!(validate_volume(255).unwrap(), 255);
    }

    #[test]
    fn converts_raw_volume_to_percent() {
        assert_eq!(FaderVolume::from_raw(0).percent, 0);
        assert_eq!(FaderVolume::from_raw(128).percent, 50);
        assert_eq!(FaderVolume::from_raw(255).percent, 100);
    }

    #[test]
    fn converts_percent_volume_to_raw() {
        assert_eq!(FaderVolume::from_percent(0).unwrap().raw, 0);
        assert_eq!(FaderVolume::from_percent(50).unwrap().raw, 128);
        assert_eq!(FaderVolume::from_percent(100).unwrap().raw, 255);
        assert!(FaderVolume::from_percent(101).is_err());
    }
}
