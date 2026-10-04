use crate::{
    DeviceConnectionInfo, DeviceProvider, DeviceReadOnlyState, DiscoverySnapshot, DriverInfo,
};
use goxlr_model::{ChannelName, DeviceIdentity, DeviceModel, FaderName, FaderState, VersionNumber};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Default)]
pub struct MockDeviceProvider {
    enabled: AtomicBool,
}

impl MockDeviceProvider {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled: AtomicBool::new(enabled),
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }
}

impl DeviceProvider for MockDeviceProvider {
    fn discover(&self) -> Result<DiscoverySnapshot, crate::DeviceError> {
        if !self.enabled() {
            return Ok(DiscoverySnapshot::default());
        }

        Ok(DiscoverySnapshot {
            devices: vec![DeviceConnectionInfo {
                identity: DeviceIdentity {
                    id: "mock:goxlr-mini:dev".to_string(),
                    model: DeviceModel::GoXlrMini,
                    vendor_id: None,
                    product_id: None,
                    manufacturer_name: Some("FriesXLR".to_string()),
                    product_name: Some("Mock GoXLR Mini".to_string()),
                    serial_number: Some("MOCK-MINI-0001".to_string()),
                    firmware_version: Some(VersionNumber {
                        major: 1,
                        minor: 3,
                        patch: Some(40),
                        build: Some(0),
                    }),
                    driver_interface: Some("mock".to_string()),
                    driver_version: None,
                    is_mock: true,
                },
                read_only_state: Some(DeviceReadOnlyState {
                    faders: vec![
                        fader(FaderName::A, ChannelName::Mic, 196, false),
                        fader(FaderName::B, ChannelName::Chat, 172, false),
                        fader(FaderName::C, ChannelName::Music, 214, false),
                        fader(FaderName::D, ChannelName::System, 160, true),
                    ],
                }),
            }],
        })
    }

    fn driver_info(&self) -> DriverInfo {
        DriverInfo {
            interface: Some("mock".to_string()),
            version: None,
            available: self.enabled(),
            last_error: None,
        }
    }
}

fn fader(name: FaderName, channel: ChannelName, volume: u8, muted: bool) -> FaderState {
    FaderState {
        name,
        assigned_channel: Some(channel),
        volume: Some(volume),
        muted: Some(muted),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_provider_can_be_toggled() {
        let provider = MockDeviceProvider::new(false);
        assert!(provider.discover().unwrap().devices.is_empty());

        provider.set_enabled(true);
        let devices = provider.discover().unwrap().devices;

        assert_eq!(devices.len(), 1);
        assert!(devices[0].identity.is_mock);
    }
}
