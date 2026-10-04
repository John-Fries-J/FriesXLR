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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FaderName {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    pub volume: Option<u8>,
    pub muted: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceState {
    pub identity: DeviceIdentity,
    pub status: ConnectionStatus,
    pub faders: Vec<FaderState>,
    pub last_seen_epoch_ms: u64,
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
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("invalid fader volume {0}; expected 0..=255")]
    InvalidFaderVolume(u16),
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
}
