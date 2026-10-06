use crate::{
    DeviceConnectionInfo, DeviceError, DeviceEvent, DeviceProvider, DeviceReadOnlyState,
    DeviceSession, DeviceSessionState, DiscoverySnapshot, DriverInfo, SessionGeneration,
};
use goxlr_model::{
    compressor_attack_option, compressor_ratio_option, compressor_release_option,
    validate_compressor_makeup_gain_db, validate_compressor_threshold_db, validate_eq_frequency,
    validate_eq_gain_db, validate_gate_threshold_db, validate_microphone_gain_db, validate_percent,
    ChannelName, CompressorState, DeEsserState, DeviceCapabilities, DeviceIdentity, DeviceModel,
    EqBandId, EqBandState, FaderMuteState, FaderName, FaderState, FaderVolume, MicrophoneState,
    MicrophoneType, NoiseGateState, RoutingRoute, RoutingState, VersionNumber,
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
                    routing: Some(state.routing.clone()),
                    microphone: Some(state.microphone.clone()),
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
            routing: Some(state.routing.clone()),
            microphone: Some(state.microphone.clone()),
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

    fn set_routing_route(&mut self, route: RoutingRoute, enabled: bool) -> Result<(), DeviceError> {
        if self.closed {
            return Err(DeviceError::SessionClosed);
        }

        self.state.lock().set_routing_route(route, enabled)
    }

    fn set_microphone_type(
        &mut self,
        microphone_type: MicrophoneType,
        confirm_phantom_power: bool,
    ) -> Result<(), DeviceError> {
        if self.closed {
            return Err(DeviceError::SessionClosed);
        }

        self.state
            .lock()
            .set_microphone_type(microphone_type, confirm_phantom_power)
    }

    fn set_microphone_gain(
        &mut self,
        microphone_type: MicrophoneType,
        gain_db: u16,
    ) -> Result<(), DeviceError> {
        if self.closed {
            return Err(DeviceError::SessionClosed);
        }

        self.state
            .lock()
            .set_microphone_gain(microphone_type, gain_db)
    }

    fn set_equalizer_band(
        &mut self,
        band_id: EqBandId,
        frequency_tenths_hz: u32,
        gain_db: i8,
    ) -> Result<(), DeviceError> {
        if self.closed {
            return Err(DeviceError::SessionClosed);
        }

        self.state
            .lock()
            .set_equalizer_band(band_id, frequency_tenths_hz, gain_db)
    }

    fn set_noise_gate(&mut self, gate: NoiseGateState) -> Result<(), DeviceError> {
        if self.closed {
            return Err(DeviceError::SessionClosed);
        }

        self.state.lock().set_noise_gate(gate)
    }

    fn set_compressor(&mut self, compressor: CompressorState) -> Result<(), DeviceError> {
        if self.closed {
            return Err(DeviceError::SessionClosed);
        }

        self.state.lock().set_compressor(compressor)
    }

    fn set_de_esser(&mut self, de_esser: DeEsserState) -> Result<(), DeviceError> {
        if self.closed {
            return Err(DeviceError::SessionClosed);
        }

        self.state.lock().set_de_esser(de_esser)
    }
}

#[derive(Debug, Clone)]
struct MockHardware {
    enabled: bool,
    connected: bool,
    identity: DeviceIdentity,
    faders: Vec<FaderState>,
    routing: RoutingState,
    microphone: MicrophoneState,
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
            routing: RoutingState::default_for_outputs(
                &DeviceCapabilities::mock().supported_routing_outputs,
            ),
            microphone: MicrophoneState::default_for_model(DeviceModel::GoXlrMini),
            events: VecDeque::new(),
        }
    }

    fn set_fader_volume(
        &mut self,
        fader: FaderName,
        volume: FaderVolume,
    ) -> Result<(), DeviceError> {
        self.ensure_connected()?;
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
        self.ensure_connected()?;
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
        self.ensure_connected()?;
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

    fn set_routing_route(&mut self, route: RoutingRoute, enabled: bool) -> Result<(), DeviceError> {
        self.ensure_connected()?;
        if !DeviceCapabilities::mock().supported_routes.contains(&route) {
            return Err(DeviceError::UnsupportedOperation(format!(
                "unsupported mock routing path: {:?}",
                route
            )));
        }

        if self.routing.is_enabled(route) == Some(enabled) {
            return Ok(());
        }

        self.routing.set_enabled(route, enabled)?;
        self.events.push_back(MockEvent::RoutingChanged {
            routing: self.routing.clone(),
        });
        Ok(())
    }

    fn set_microphone_type(
        &mut self,
        microphone_type: MicrophoneType,
        confirm_phantom_power: bool,
    ) -> Result<(), DeviceError> {
        self.ensure_connected()?;
        if microphone_type.has_phantom_power() && !confirm_phantom_power {
            return Err(DeviceError::UnsupportedOperation(
                "condenser microphone selection requires explicit phantom power confirmation"
                    .to_string(),
            ));
        }

        if self.microphone.setup.microphone_type == microphone_type {
            return Ok(());
        }

        self.microphone.setup.microphone_type = microphone_type;
        self.microphone.setup.phantom_power_enabled = microphone_type.has_phantom_power();
        self.events.push_back(MockEvent::MicrophoneChanged {
            microphone: self.microphone.clone(),
        });
        Ok(())
    }

    fn set_microphone_gain(
        &mut self,
        microphone_type: MicrophoneType,
        gain_db: u16,
    ) -> Result<(), DeviceError> {
        self.ensure_connected()?;
        validate_microphone_gain_db(gain_db)?;
        let gain = self
            .microphone
            .gain_for_mut(microphone_type)
            .ok_or_else(|| {
                DeviceError::UnsupportedOperation(format!(
                    "unsupported mock microphone type: {microphone_type}"
                ))
            })?;

        if gain.hardware_db == gain_db {
            return Ok(());
        }

        gain.hardware_db = gain_db;
        self.events.push_back(MockEvent::MicrophoneChanged {
            microphone: self.microphone.clone(),
        });
        Ok(())
    }

    fn set_equalizer_band(
        &mut self,
        band_id: EqBandId,
        frequency_tenths_hz: u32,
        gain_db: i8,
    ) -> Result<(), DeviceError> {
        self.ensure_connected()?;
        validate_eq_gain_db(gain_db)?;
        let capability = DeviceCapabilities::mock()
            .eq_bands
            .into_iter()
            .find(|capability| capability.id == band_id)
            .ok_or_else(|| {
                DeviceError::UnsupportedOperation(format!("unsupported mock EQ band: {band_id:?}"))
            })?;
        validate_eq_frequency(&capability, frequency_tenths_hz)?;

        let band = self.microphone.eq_band_mut(band_id).ok_or_else(|| {
            DeviceError::DeviceUnavailable(format!("missing mock EQ band {band_id:?}"))
        })?;
        if band.frequency_tenths_hz == frequency_tenths_hz && band.gain_db == gain_db {
            return Ok(());
        }

        *band = EqBandState {
            id: band_id,
            frequency_tenths_hz,
            gain_db,
        };
        self.events.push_back(MockEvent::MicrophoneChanged {
            microphone: self.microphone.clone(),
        });
        Ok(())
    }

    fn set_noise_gate(&mut self, gate: NoiseGateState) -> Result<(), DeviceError> {
        self.ensure_connected()?;
        validate_gate_threshold_db(gate.threshold_db)?;
        validate_percent(gate.attenuation_percent)?;
        if goxlr_model::gate_time_option(gate.attack.index).is_none() {
            return Err(DeviceError::UnsupportedOperation(format!(
                "unsupported gate attack index: {}",
                gate.attack.index
            )));
        }
        if goxlr_model::gate_time_option(gate.release.index).is_none() {
            return Err(DeviceError::UnsupportedOperation(format!(
                "unsupported gate release index: {}",
                gate.release.index
            )));
        }

        if self.microphone.gate == gate {
            return Ok(());
        }

        self.microphone.gate = gate;
        self.events.push_back(MockEvent::MicrophoneChanged {
            microphone: self.microphone.clone(),
        });
        Ok(())
    }

    fn set_compressor(&mut self, compressor: CompressorState) -> Result<(), DeviceError> {
        self.ensure_connected()?;
        validate_compressor_threshold_db(compressor.threshold_db)?;
        validate_compressor_makeup_gain_db(compressor.makeup_gain_db)?;
        if compressor_ratio_option(compressor.ratio.index).is_none() {
            return Err(DeviceError::UnsupportedOperation(format!(
                "unsupported compressor ratio index: {}",
                compressor.ratio.index
            )));
        }
        if compressor_attack_option(compressor.attack.index).is_none() {
            return Err(DeviceError::UnsupportedOperation(format!(
                "unsupported compressor attack index: {}",
                compressor.attack.index
            )));
        }
        if compressor_release_option(compressor.release.index).is_none() {
            return Err(DeviceError::UnsupportedOperation(format!(
                "unsupported compressor release index: {}",
                compressor.release.index
            )));
        }

        if self.microphone.compressor == compressor {
            return Ok(());
        }

        self.microphone.compressor = compressor;
        self.events.push_back(MockEvent::MicrophoneChanged {
            microphone: self.microphone.clone(),
        });
        Ok(())
    }

    fn set_de_esser(&mut self, de_esser: DeEsserState) -> Result<(), DeviceError> {
        self.ensure_connected()?;
        validate_percent(de_esser.amount_percent)?;
        if self.microphone.de_esser == de_esser {
            return Ok(());
        }

        self.microphone.de_esser = de_esser;
        self.events.push_back(MockEvent::MicrophoneChanged {
            microphone: self.microphone.clone(),
        });
        Ok(())
    }

    fn ensure_connected(&self) -> Result<(), DeviceError> {
        if self.connected {
            Ok(())
        } else {
            Err(DeviceError::DeviceDisconnected)
        }
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
    RoutingChanged {
        routing: RoutingState,
    },
    MicrophoneChanged {
        microphone: MicrophoneState,
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
            MockEvent::RoutingChanged { routing } => DeviceEvent::RoutingChanged {
                device_id,
                generation,
                routing,
            },
            MockEvent::MicrophoneChanged { microphone } => DeviceEvent::MicrophoneChanged {
                device_id,
                generation,
                microphone,
            },
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
    fn mock_session_reports_assignment_and_mute_events() {
        let provider = MockDeviceProvider::new(true);
        let identity = provider.discover().unwrap().devices[0].identity.clone();
        let mut session = provider.open_session(&identity, 7).unwrap();

        session
            .set_fader_assignment(FaderName::A, ChannelName::Game)
            .unwrap();
        assert_eq!(
            session.poll_event().unwrap().unwrap(),
            DeviceEvent::FaderAssignmentChanged {
                device_id: identity.id.clone(),
                generation: 7,
                fader: FaderName::A,
                channel: Some(ChannelName::Game),
            }
        );

        session.set_fader_mute(FaderName::A, true).unwrap();
        assert_eq!(
            session.poll_event().unwrap().unwrap(),
            DeviceEvent::FaderMuteStateChanged {
                device_id: identity.id.clone(),
                generation: 7,
                fader: FaderName::A,
                mute_state: Some(FaderMuteState::MutedToAll),
                muted: Some(true),
            }
        );
        assert_eq!(
            session.poll_event().unwrap().unwrap(),
            DeviceEvent::FaderMuteButtonChanged {
                device_id: identity.id,
                generation: 7,
                fader: FaderName::A,
                pressed: true,
            }
        );
    }

    #[test]
    fn mock_session_reports_routing_events() {
        let provider = MockDeviceProvider::new(true);
        let identity = provider.discover().unwrap().devices[0].identity.clone();
        let mut session = provider.open_session(&identity, 11).unwrap();
        let route = RoutingRoute {
            input: goxlr_model::RoutingInput::Music,
            output: goxlr_model::RoutingOutput::BroadcastMix,
        };

        session.set_routing_route(route, false).unwrap();

        let event = session.poll_event().unwrap().unwrap();
        match event {
            DeviceEvent::RoutingChanged {
                device_id,
                generation,
                routing,
            } => {
                assert_eq!(device_id, identity.id);
                assert_eq!(generation, 11);
                assert_eq!(routing.is_enabled(route), Some(false));
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }

    #[test]
    fn mock_session_reports_microphone_events() {
        let provider = MockDeviceProvider::new(true);
        let identity = provider.discover().unwrap().devices[0].identity.clone();
        let mut session = provider.open_session(&identity, 12).unwrap();

        session
            .set_microphone_gain(MicrophoneType::Dynamic, 45)
            .unwrap();

        let event = session.poll_event().unwrap().unwrap();
        match event {
            DeviceEvent::MicrophoneChanged {
                device_id,
                generation,
                microphone,
            } => {
                assert_eq!(device_id, identity.id);
                assert_eq!(generation, 12);
                assert_eq!(microphone.setup.gains[0].hardware_db, 45);
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }

    #[test]
    fn mock_rejects_unsupported_and_disconnected_commands() {
        let provider = MockDeviceProvider::new(true);
        let identity = provider.discover().unwrap().devices[0].identity.clone();
        let mut session = provider.open_session(&identity, 13).unwrap();

        assert!(matches!(
            session.set_routing_route(
                RoutingRoute {
                    input: goxlr_model::RoutingInput::Chat,
                    output: goxlr_model::RoutingOutput::ChatMic,
                },
                true,
            ),
            Err(DeviceError::UnsupportedOperation(_))
        ));
        assert!(matches!(
            session.set_microphone_type(MicrophoneType::Condenser, false),
            Err(DeviceError::UnsupportedOperation(_))
        ));

        provider.simulate_disconnect();
        assert!(matches!(
            session.set_microphone_gain(MicrophoneType::Dynamic, 20),
            Err(DeviceError::DeviceDisconnected)
        ));
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
