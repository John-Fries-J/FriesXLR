use crate::{
    DeviceConnectionInfo, DeviceError, DeviceEvent, DeviceProvider, DeviceReadOnlyState,
    DeviceSession, DeviceSessionState, DiscoverySnapshot, DriverInfo, SessionGeneration,
};
use goxlr_model::{
    ChannelName, DeviceCapabilities, DeviceIdentity, DeviceModel, FaderMuteState, FaderName,
    FaderState, FaderVolume, VersionNumber,
};
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Debug)]
pub struct MockDeviceProvider {
    state: Arc<Mutex<MockHardware>>,
}

impl MockDeviceProvider {
    pub fn new(enabled: bool) -> Self {
        Self {
            state: Arc::new(Mutex::new(MockHardware::new(enabled))),
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        let mut state = self.state.lock();
        state.enabled = enabled;
        state.connected = enabled;
    }

    pub fn enabled(&self) -> bool {
        self.state.lock().enabled
    }

    pub fn simulate_disconnect(&self) {
        let mut state = self.state.lock();
        state.connected = false;
        state.events.push_back(MockEvent::Disconnected);
    }

    pub fn simulate_reconnect(&self) {
        let mut state = self.state.lock();
        state.connected = state.enabled;
    }

    pub fn simulate_fader_volume(
        &self,
        fader: FaderName,
        volume: FaderVolume,
    ) -> Result<(), DeviceError> {
        let mut state = self.state.lock();
        state.set_fader_volume(fader, volume)
    }

    pub fn simulate_fader_mute(&self, fader: FaderName, muted: bool) -> Result<(), DeviceError> {
        let mut state = self.state.lock();
        state.set_fader_mute(fader, muted)
    }

    pub fn simulate_fader_assignment(
        &self,
        fader: FaderName,
        channel: ChannelName,
    ) -> Result<(), DeviceError> {
        let mut state = self.state.lock();
        state.set_fader_assignment(fader, channel)
    }
}

impl DeviceProvider for MockDeviceProvider {
    fn discover(&self) -> Result<DiscoverySnapshot, DeviceError> {
        let state = self.state.lock();
        if !state.enabled || !state.connected {
            return Ok(DiscoverySnapshot::default());
        }

        Ok(DiscoverySnapshot {
            devices: vec![DeviceConnectionInfo {
                identity: state.identity.clone(),
                read_only_state: Some(DeviceReadOnlyState {
                    capabilities: DeviceCapabilities::mock(),
                    faders: state.faders.clone(),
                }),
            }],
        })
    }

    fn open_session(
        &self,
        identity: &DeviceIdentity,
        generation: SessionGeneration,
    ) -> Result<Box<dyn DeviceSession>, DeviceError> {
        let state = self.state.lock();
        if !state.enabled || !state.connected || identity.id != state.identity.id {
            return Err(DeviceError::DeviceUnavailable(identity.id.clone()));
        }
        drop(state);

        Ok(Box::new(MockDeviceSession {
            generation,
            state: self.state.clone(),
            closed: false,
        }))
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

#[derive(Debug)]
struct MockDeviceSession {
    generation: SessionGeneration,
    state: Arc<Mutex<MockHardware>>,
    closed: bool,
}

impl DeviceSession for MockDeviceSession {
    fn generation(&self) -> SessionGeneration {
        self.generation
    }

    fn current_state(&self) -> Result<DeviceSessionState, DeviceError> {
        if self.closed {
            return Err(DeviceError::SessionClosed);
        }

        let state = self.state.lock();
        if !state.connected {
            return Err(DeviceError::DeviceDisconnected);
        }

        Ok(DeviceSessionState {
            identity: state.identity.clone(),
            capabilities: DeviceCapabilities::mock(),
            faders: state.faders.clone(),
        })
    }

    fn poll_event(&mut self) -> Result<Option<DeviceEvent>, DeviceError> {
        if self.closed {
            return Ok(None);
        }

        let mut state = self.state.lock();
        let Some(event) = state.events.pop_front() else {
            return Ok(None);
        };

        Ok(Some(event.into_device_event(
            state.identity.id.clone(),
            self.generation,
        )))
    }

    fn close(&mut self) {
        self.closed = true;
    }

    fn set_fader_volume(
        &mut self,
        fader: FaderName,
        volume: FaderVolume,
    ) -> Result<(), DeviceError> {
        if self.closed {
            return Err(DeviceError::SessionClosed);
        }

        self.state.lock().set_fader_volume(fader, volume)
    }

    fn set_fader_mute(&mut self, fader: FaderName, muted: bool) -> Result<(), DeviceError> {
        if self.closed {
            return Err(DeviceError::SessionClosed);
        }

        self.state.lock().set_fader_mute(fader, muted)
    }

    fn set_fader_assignment(
        &mut self,
        fader: FaderName,
        channel: ChannelName,
    ) -> Result<(), DeviceError> {
        if self.closed {
            return Err(DeviceError::SessionClosed);
        }

        self.state.lock().set_fader_assignment(fader, channel)
    }
}

#[derive(Debug, Clone)]
struct MockHardware {
    enabled: bool,
    connected: bool,
    identity: DeviceIdentity,
    faders: Vec<FaderState>,
    events: VecDeque<MockEvent>,
}

impl MockHardware {
    fn new(enabled: bool) -> Self {
        Self {
            enabled,
            connected: enabled,
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
            faders: vec![
                fader(FaderName::A, ChannelName::Mic, 196, false),
                fader(FaderName::B, ChannelName::Chat, 172, false),
                fader(FaderName::C, ChannelName::Music, 214, false),
                fader(FaderName::D, ChannelName::System, 160, true),
            ],
            events: VecDeque::new(),
        }
    }

    fn set_fader_volume(
        &mut self,
        fader: FaderName,
        volume: FaderVolume,
    ) -> Result<(), DeviceError> {
        let state = self.fader_mut(fader)?;
        if state.volume == Some(volume) {
            return Ok(());
        }

        state.volume = Some(volume);
        self.events
            .push_back(MockEvent::FaderVolumeChanged { fader, volume });
        Ok(())
    }

    fn set_fader_mute(&mut self, fader: FaderName, muted: bool) -> Result<(), DeviceError> {
        let state = self.fader_mut(fader)?;
        let mute_state = if muted {
            FaderMuteState::MutedToAll
        } else {
            FaderMuteState::Unmuted
        };

        if state.muted == Some(muted) && state.mute_state == Some(mute_state) {
            return Ok(());
        }

        state.muted = Some(muted);
        state.mute_state = Some(mute_state);
        state.mute_button_pressed = Some(muted);
        self.events.push_back(MockEvent::FaderMuteStateChanged {
            fader,
            mute_state: Some(mute_state),
            muted: Some(muted),
        });
        self.events.push_back(MockEvent::FaderMuteButtonChanged {
            fader,
            pressed: muted,
        });
        Ok(())
    }

    fn set_fader_assignment(
        &mut self,
        fader: FaderName,
        channel: ChannelName,
    ) -> Result<(), DeviceError> {
        if !DeviceCapabilities::mock()
            .supported_assignment_channels
            .contains(&channel)
        {
            return Err(DeviceError::UnsupportedOperation(format!(
                "unsupported mock channel assignment: {channel}"
            )));
        }

        let state = self.fader_mut(fader)?;
        if state.assigned_channel == Some(channel) {
            return Ok(());
        }

        state.assigned_channel = Some(channel);
        self.events
            .push_back(MockEvent::FaderAssignmentChanged { fader, channel });
        Ok(())
    }

    fn fader_mut(&mut self, fader: FaderName) -> Result<&mut FaderState, DeviceError> {
        self.faders
            .iter_mut()
            .find(|state| state.name == fader)
            .ok_or_else(|| DeviceError::DeviceUnavailable(format!("missing mock fader {fader:?}")))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MockEvent {
    FaderVolumeChanged {
        fader: FaderName,
        volume: FaderVolume,
    },
    FaderMuteStateChanged {
        fader: FaderName,
        mute_state: Option<FaderMuteState>,
        muted: Option<bool>,
    },
    FaderMuteButtonChanged {
        fader: FaderName,
        pressed: bool,
    },
    FaderAssignmentChanged {
        fader: FaderName,
        channel: ChannelName,
    },
    Disconnected,
}

impl MockEvent {
    fn into_device_event(self, device_id: String, generation: SessionGeneration) -> DeviceEvent {
        match self {
            MockEvent::FaderVolumeChanged { fader, volume } => DeviceEvent::FaderVolumeChanged {
                device_id,
                generation,
                fader,
                volume,
            },
            MockEvent::FaderMuteStateChanged {
                fader,
                mute_state,
                muted,
            } => DeviceEvent::FaderMuteStateChanged {
                device_id,
                generation,
                fader,
                mute_state,
                muted,
            },
            MockEvent::FaderMuteButtonChanged { fader, pressed } => {
                DeviceEvent::FaderMuteButtonChanged {
                    device_id,
                    generation,
                    fader,
                    pressed,
                }
            }
            MockEvent::FaderAssignmentChanged { fader, channel } => {
                DeviceEvent::FaderAssignmentChanged {
                    device_id,
                    generation,
                    fader,
                    channel: Some(channel),
                }
            }
            MockEvent::Disconnected => DeviceEvent::Disconnected {
                device_id,
                generation,
            },
        }
    }
}

fn fader(name: FaderName, channel: ChannelName, raw_volume: u8, muted: bool) -> FaderState {
    let mute_state = if muted {
        FaderMuteState::MutedToAll
    } else {
        FaderMuteState::Unmuted
    };

    FaderState {
        name,
        assigned_channel: Some(channel),
        volume: Some(FaderVolume::from_raw(raw_volume)),
        mute_state: Some(mute_state),
        mute_function: Some(goxlr_model::MuteFunction::All),
        muted: Some(muted),
        mute_button_pressed: Some(muted),
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

    #[test]
    fn mock_session_reports_volume_events() {
        let provider = MockDeviceProvider::new(true);
        let identity = provider.discover().unwrap().devices[0].identity.clone();
        let mut session = provider.open_session(&identity, 42).unwrap();

        session
            .set_fader_volume(FaderName::A, FaderVolume::from_raw(10))
            .unwrap();

        let event = session.poll_event().unwrap().unwrap();
        assert_eq!(
            event,
            DeviceEvent::FaderVolumeChanged {
                device_id: identity.id,
                generation: 42,
                fader: FaderName::A,
                volume: FaderVolume::from_raw(10),
            }
        );
    }

    #[test]
    fn mock_session_closes_cleanly() {
        let provider = MockDeviceProvider::new(true);
        let identity = provider.discover().unwrap().devices[0].identity.clone();
        let mut session = provider.open_session(&identity, 1).unwrap();

        session.close();

        assert!(matches!(
            session.current_state(),
            Err(DeviceError::SessionClosed)
        ));
    }
}
