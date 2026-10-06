//! Protocol identifiers and packet helpers used by FriesXLR.
//!
//! These constants and packet shapes are adapted from the MIT-licensed GoXLR Utility
//! project:
//! https://github.com/GoXLR-on-Linux/goxlr-utility
//! See `NOTICE.md` for attribution.

use goxlr_model::{
    validate_compressor_makeup_gain_db, validate_compressor_threshold_db, validate_eq_gain_db,
    validate_gate_threshold_db, validate_microphone_gain_db, validate_percent, ChannelName,
    CompressorRatioOption, DeviceModel, EqBandId, FaderMuteState, FaderName, FaderVolume,
    MicrophoneType, MuteFunction, RoutingInput, RoutingOutput, RoutingRoute, RoutingRouteState,
    RoutingState, TimeOption, VersionNumber,
};

pub const TC_HELICON_VENDOR_ID: u16 = 0x1220;
pub const GOXLR_PRODUCT_ID: u16 = 0x8fe0;
pub const GOXLR_MINI_PRODUCT_ID: u16 = 0x8fe4;
pub const FRAME_HEADER_LEN: usize = 16;
pub const MAX_RESPONSE_LEN: usize = 1040;

pub fn model_from_product_id(product_id: u16) -> DeviceModel {
    match product_id {
        GOXLR_PRODUCT_ID => DeviceModel::GoXlr,
        GOXLR_MINI_PRODUCT_ID => DeviceModel::GoXlrMini,
        _ => DeviceModel::Unknown,
    }
}

pub fn is_known_goxlr_device(vendor_id: u16, product_id: u16) -> bool {
    vendor_id == TC_HELICON_VENDOR_ID
        && matches!(product_id, GOXLR_PRODUCT_ID | GOXLR_MINI_PRODUCT_ID)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolCommand {
    ResetCommandIndex,
    GetButtonStates,
    GetHardwareInfo(HardwareInfoCommand),
    SetChannelVolume(ChannelName),
    SetChannelState(ChannelName),
    SetFader(FaderName),
    SetRouting(ProtocolRoutingInput),
    SetMicrophoneParameters,
    SetEffectParameters,
}

impl ProtocolCommand {
    pub fn command_id(self) -> Result<u32, ProtocolError> {
        Ok(match self {
            ProtocolCommand::ResetCommandIndex => 0,
            ProtocolCommand::GetButtonStates => 0x800 << 12,
            ProtocolCommand::GetHardwareInfo(command) => (0x80f << 12) | command as u32,
            ProtocolCommand::SetChannelVolume(channel) => {
                (0x806 << 12) | channel_protocol_index(channel)? as u32
            }
            ProtocolCommand::SetChannelState(channel) => {
                (0x809 << 12) | channel_protocol_index(channel)? as u32
            }
            ProtocolCommand::SetFader(fader) => (0x805 << 12) | fader.index() as u32,
            ProtocolCommand::SetRouting(input) => (0x804 << 12) | input.id() as u32,
            ProtocolCommand::SetMicrophoneParameters => 0x80b << 12,
            ProtocolCommand::SetEffectParameters => 0x801 << 12,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareInfoCommand {
    FirmwareVersion = 0,
    SerialNumber = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolRoutingInput {
    MicrophoneLeft,
    MicrophoneRight,
    MusicLeft,
    MusicRight,
    GameLeft,
    GameRight,
    ChatLeft,
    ChatRight,
    ConsoleLeft,
    ConsoleRight,
    LineInLeft,
    LineInRight,
    SystemLeft,
    SystemRight,
    SampleLeft,
    SampleRight,
}

impl ProtocolRoutingInput {
    pub fn id(self) -> u8 {
        match self {
            ProtocolRoutingInput::MicrophoneLeft => 0x02,
            ProtocolRoutingInput::MicrophoneRight => 0x03,
            ProtocolRoutingInput::LineInLeft => 0x04,
            ProtocolRoutingInput::LineInRight => 0x05,
            ProtocolRoutingInput::ConsoleLeft => 0x06,
            ProtocolRoutingInput::ConsoleRight => 0x07,
            ProtocolRoutingInput::SystemLeft => 0x08,
            ProtocolRoutingInput::SystemRight => 0x09,
            ProtocolRoutingInput::GameLeft => 0x0a,
            ProtocolRoutingInput::GameRight => 0x0b,
            ProtocolRoutingInput::ChatLeft => 0x0c,
            ProtocolRoutingInput::ChatRight => 0x0d,
            ProtocolRoutingInput::MusicLeft => 0x0e,
            ProtocolRoutingInput::MusicRight => 0x0f,
            ProtocolRoutingInput::SampleLeft => 0x10,
            ProtocolRoutingInput::SampleRight => 0x11,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingSide {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingPayload {
    pub input: ProtocolRoutingInput,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicrophoneParamKey {
    MicType = 0x000,
    DynamicGain = 0x001,
    CondenserGain = 0x002,
    JackGain = 0x003,
    GateThreshold = 0x30200,
    GateAttack = 0x30400,
    GateRelease = 0x30600,
    GateAttenuation = 0x30900,
    CompressorThreshold = 0x60200,
    CompressorRatio = 0x60300,
    CompressorAttack = 0x60400,
    CompressorRelease = 0x60600,
    CompressorMakeUpGain = 0x60700,
    Equalizer90HzFrequency = 0x40000,
    Equalizer90HzGain = 0x40001,
    Equalizer250HzFrequency = 0x40003,
    Equalizer250HzGain = 0x40004,
    Equalizer500HzFrequency = 0x40006,
    Equalizer500HzGain = 0x40007,
    Equalizer1KHzFrequency = 0x50000,
    Equalizer1KHzGain = 0x50001,
    Equalizer3KHzFrequency = 0x50003,
    Equalizer3KHzGain = 0x50004,
    Equalizer8KHzFrequency = 0x50006,
    Equalizer8KHzGain = 0x50007,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectKey {
    GateThreshold = 0x0011,
    GateEnabled = 0x0014,
    GateAttenuation = 0x0015,
    GateAttack = 0x0016,
    GateRelease = 0x0017,
    Equalizer31HzFrequency = 0x0126,
    Equalizer31HzGain = 0x0127,
    Equalizer63HzFrequency = 0x00f8,
    Equalizer63HzGain = 0x00f9,
    Equalizer125HzFrequency = 0x0113,
    Equalizer125HzGain = 0x0114,
    Equalizer250HzFrequency = 0x0129,
    Equalizer250HzGain = 0x012a,
    Equalizer500HzFrequency = 0x0116,
    Equalizer500HzGain = 0x0117,
    Equalizer1KHzFrequency = 0x011d,
    Equalizer1KHzGain = 0x011e,
    Equalizer2KHzFrequency = 0x012c,
    Equalizer2KHzGain = 0x012d,
    Equalizer4KHzFrequency = 0x0120,
    Equalizer4KHzGain = 0x0121,
    Equalizer8KHzFrequency = 0x0109,
    Equalizer8KHzGain = 0x010a,
    Equalizer16KHzFrequency = 0x012f,
    Equalizer16KHzGain = 0x0130,
    CompressorThreshold = 0x013d,
    CompressorRatio = 0x013c,
    CompressorAttack = 0x013e,
    CompressorRelease = 0x013f,
    CompressorMakeUpGain = 0x0140,
    DeEsser = 0x000b,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelState {
    Unmuted,
    Muted,
}

impl ChannelState {
    pub fn id(self) -> u8 {
        match self {
            ChannelState::Unmuted => 0x00,
            ChannelState::Muted => 0x01,
        }
    }
}

pub fn routing_input_sides(input: RoutingInput) -> (ProtocolRoutingInput, ProtocolRoutingInput) {
    match input {
        RoutingInput::Microphone => (
            ProtocolRoutingInput::MicrophoneLeft,
            ProtocolRoutingInput::MicrophoneRight,
        ),
        RoutingInput::Chat => (
            ProtocolRoutingInput::ChatLeft,
            ProtocolRoutingInput::ChatRight,
        ),
        RoutingInput::Music => (
            ProtocolRoutingInput::MusicLeft,
            ProtocolRoutingInput::MusicRight,
        ),
        RoutingInput::Game => (
            ProtocolRoutingInput::GameLeft,
            ProtocolRoutingInput::GameRight,
        ),
        RoutingInput::Console => (
            ProtocolRoutingInput::ConsoleLeft,
            ProtocolRoutingInput::ConsoleRight,
        ),
        RoutingInput::LineIn => (
            ProtocolRoutingInput::LineInLeft,
            ProtocolRoutingInput::LineInRight,
        ),
        RoutingInput::System => (
            ProtocolRoutingInput::SystemLeft,
            ProtocolRoutingInput::SystemRight,
        ),
        RoutingInput::Sample => (
            ProtocolRoutingInput::SampleLeft,
            ProtocolRoutingInput::SampleRight,
        ),
    }
}

pub fn build_routing_payloads(
    routing: &RoutingState,
    input: RoutingInput,
    mix2_enabled: bool,
) -> Result<[RoutingPayload; 2], ProtocolError> {
    let (left_input, right_input) = routing_input_sides(input);
    let mut left = routing_payload_buffer(mix2_enabled);
    let mut right = routing_payload_buffer(mix2_enabled);

    for route in routing
        .routes
        .iter()
        .filter(|state| state.route.input == input)
    {
        if !route.enabled {
            continue;
        }

        validate_routing_output(route.route.output, mix2_enabled)?;
        let (left_position, right_position) =
            routing_output_positions(route.route.output, mix2_enabled);
        left[left_position] = 0x20;
        right[right_position] = 0x20;
    }

    Ok([
        RoutingPayload {
            input: left_input,
            body: left,
        },
        RoutingPayload {
            input: right_input,
            body: right,
        },
    ])
}

pub fn parse_routing_payloads(
    input: RoutingInput,
    left: &[u8],
    right: &[u8],
    mix2_enabled: bool,
) -> Result<Vec<RoutingRouteState>, ProtocolError> {
    let expected_len = routing_payload_len(mix2_enabled);
    if left.len() != expected_len || right.len() != expected_len {
        return Err(ProtocolError::MalformedResponse {
            reason: format!(
                "routing payload length mismatch: expected {expected_len}, received left={}, right={}",
                left.len(),
                right.len()
            ),
        });
    }

    let outputs = if mix2_enabled {
        RoutingOutput::WITH_STREAM_MIX_2.as_slice()
    } else {
        RoutingOutput::BASE.as_slice()
    };

    let mut routes = Vec::new();
    for output in outputs {
        let route = RoutingRoute {
            input,
            output: *output,
        };
        if !goxlr_model::is_supported_route(route) {
            continue;
        }

        let (left_position, right_position) = routing_output_positions(*output, mix2_enabled);
        routes.push(RoutingRouteState {
            route,
            enabled: left[left_position] != 0 || right[right_position] != 0,
        });
    }

    Ok(routes)
}

fn routing_payload_buffer(mix2_enabled: bool) -> Vec<u8> {
    vec![0; routing_payload_len(mix2_enabled)]
}

fn routing_payload_len(mix2_enabled: bool) -> usize {
    if mix2_enabled {
        26
    } else {
        22
    }
}

fn validate_routing_output(output: RoutingOutput, mix2_enabled: bool) -> Result<(), ProtocolError> {
    if output == RoutingOutput::StreamMix2 && !mix2_enabled {
        return Err(ProtocolError::UnsupportedRoutingOutput(output));
    }
    Ok(())
}

fn routing_output_positions(output: RoutingOutput, mix2_enabled: bool) -> (usize, usize) {
    match output {
        RoutingOutput::Headphones => (1, 3),
        RoutingOutput::BroadcastMix => (5, 7),
        RoutingOutput::ChatMic => (9, 11),
        RoutingOutput::Sampler => (13, 15),
        RoutingOutput::LineOut => (17, 19),
        RoutingOutput::StreamMix2 => {
            if mix2_enabled {
                (21, 23)
            } else {
                (usize::MAX, usize::MAX)
            }
        }
    }
}

pub fn microphone_gain_params(
    microphone_type: MicrophoneType,
    gain_db: u16,
) -> Result<Vec<(MicrophoneParamKey, [u8; 4])>, ProtocolError> {
    validate_microphone_gain_db(gain_db)?;
    Ok(vec![
        (
            MicrophoneParamKey::MicType,
            microphone_type_bytes(microphone_type),
        ),
        (
            gain_param_for_type(microphone_type),
            microphone_gain_bytes(gain_db)?,
        ),
    ])
}

pub fn microphone_type_param(microphone_type: MicrophoneType) -> (MicrophoneParamKey, [u8; 4]) {
    (
        MicrophoneParamKey::MicType,
        microphone_type_bytes(microphone_type),
    )
}

pub fn microphone_type_bytes(microphone_type: MicrophoneType) -> [u8; 4] {
    if microphone_type.has_phantom_power() {
        [0x01, 0, 0, 0]
    } else {
        [0, 0, 0, 0]
    }
}

pub fn microphone_gain_bytes(gain_db: u16) -> Result<[u8; 4], ProtocolError> {
    let gain_db = validate_microphone_gain_db(gain_db)?;
    let mut value = [0; 4];
    value[2..4].copy_from_slice(&gain_db.to_le_bytes());
    Ok(value)
}

pub fn gain_param_for_type(microphone_type: MicrophoneType) -> MicrophoneParamKey {
    match microphone_type {
        MicrophoneType::Dynamic => MicrophoneParamKey::DynamicGain,
        MicrophoneType::Condenser => MicrophoneParamKey::CondenserGain,
        MicrophoneType::Jack => MicrophoneParamKey::JackGain,
    }
}

pub fn encode_microphone_params(params: &[(MicrophoneParamKey, [u8; 4])]) -> Vec<u8> {
    let mut data = Vec::with_capacity(params.len() * 8);
    for (key, value) in params {
        data.extend_from_slice(&(*key as u32).to_le_bytes());
        data.extend_from_slice(value);
    }
    data
}

pub fn encode_effect_params(params: &[(EffectKey, i32)]) -> Vec<u8> {
    let mut data = Vec::with_capacity(params.len() * 8);
    for (key, value) in params {
        data.extend_from_slice(&(*key as u32).to_le_bytes());
        data.extend_from_slice(&value.to_le_bytes());
    }
    data
}

pub fn f32_param_bytes(value: f32) -> [u8; 4] {
    value.to_le_bytes()
}

pub fn i8_as_f32_param_bytes(value: i8) -> [u8; 4] {
    (value as f32).to_le_bytes()
}

pub fn u8_as_f32_param_bytes(value: u8) -> [u8; 4] {
    (value as f32).to_le_bytes()
}

pub fn eq_effect_keys(band_id: EqBandId) -> Result<(EffectKey, EffectKey), ProtocolError> {
    Ok(match band_id {
        EqBandId::Eq31Hz => (
            EffectKey::Equalizer31HzFrequency,
            EffectKey::Equalizer31HzGain,
        ),
        EqBandId::Eq63Hz => (
            EffectKey::Equalizer63HzFrequency,
            EffectKey::Equalizer63HzGain,
        ),
        EqBandId::Eq125Hz => (
            EffectKey::Equalizer125HzFrequency,
            EffectKey::Equalizer125HzGain,
        ),
        EqBandId::Eq250Hz => (
            EffectKey::Equalizer250HzFrequency,
            EffectKey::Equalizer250HzGain,
        ),
        EqBandId::Eq500Hz => (
            EffectKey::Equalizer500HzFrequency,
            EffectKey::Equalizer500HzGain,
        ),
        EqBandId::Eq1KHz => (
            EffectKey::Equalizer1KHzFrequency,
            EffectKey::Equalizer1KHzGain,
        ),
        EqBandId::Eq2KHz => (
            EffectKey::Equalizer2KHzFrequency,
            EffectKey::Equalizer2KHzGain,
        ),
        EqBandId::Eq4KHz => (
            EffectKey::Equalizer4KHzFrequency,
            EffectKey::Equalizer4KHzGain,
        ),
        EqBandId::Eq8KHz => (
            EffectKey::Equalizer8KHzFrequency,
            EffectKey::Equalizer8KHzGain,
        ),
        EqBandId::Eq16KHz => (
            EffectKey::Equalizer16KHzFrequency,
            EffectKey::Equalizer16KHzGain,
        ),
        other => return Err(ProtocolError::UnsupportedEqBand(other)),
    })
}

pub fn mini_eq_param_keys(
    band_id: EqBandId,
) -> Result<(MicrophoneParamKey, MicrophoneParamKey), ProtocolError> {
    Ok(match band_id {
        EqBandId::MiniEq90Hz => (
            MicrophoneParamKey::Equalizer90HzFrequency,
            MicrophoneParamKey::Equalizer90HzGain,
        ),
        EqBandId::MiniEq250Hz => (
            MicrophoneParamKey::Equalizer250HzFrequency,
            MicrophoneParamKey::Equalizer250HzGain,
        ),
        EqBandId::MiniEq500Hz => (
            MicrophoneParamKey::Equalizer500HzFrequency,
            MicrophoneParamKey::Equalizer500HzGain,
        ),
        EqBandId::MiniEq1KHz => (
            MicrophoneParamKey::Equalizer1KHzFrequency,
            MicrophoneParamKey::Equalizer1KHzGain,
        ),
        EqBandId::MiniEq3KHz => (
            MicrophoneParamKey::Equalizer3KHzFrequency,
            MicrophoneParamKey::Equalizer3KHzGain,
        ),
        EqBandId::MiniEq8KHz => (
            MicrophoneParamKey::Equalizer8KHzFrequency,
            MicrophoneParamKey::Equalizer8KHzGain,
        ),
        other => return Err(ProtocolError::UnsupportedEqBand(other)),
    })
}

pub fn eq_frequency_effect_value(frequency_tenths_hz: u32) -> i32 {
    let freq = frequency_tenths_hz as f32 / 10.0;
    (24.0 * (freq / 20.0).log2()).round() as i32
}

pub fn eq_gain_effect_value(gain_db: i8) -> Result<i32, ProtocolError> {
    Ok(validate_eq_gain_db(gain_db)? as i32)
}

pub fn eq_mini_frequency_param_value(frequency_tenths_hz: u32) -> [u8; 4] {
    f32_param_bytes(frequency_tenths_hz as f32 / 10.0)
}

pub fn eq_mini_gain_param_value(gain_db: i8) -> Result<[u8; 4], ProtocolError> {
    Ok(i8_as_f32_param_bytes(validate_eq_gain_db(gain_db)?))
}

pub fn gate_threshold_values(threshold_db: i8) -> Result<([u8; 4], i32), ProtocolError> {
    let threshold_db = validate_gate_threshold_db(threshold_db)?;
    Ok((i8_as_f32_param_bytes(threshold_db), threshold_db as i32))
}

pub fn gate_attenuation_db_from_percent(percent: u8) -> Result<i8, ProtocolError> {
    validate_percent(percent)?;
    const GATE_ATTENUATION: [i8; 26] = [
        -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20, -21, -22, -23, -24,
        -25, -26, -27, -28, -30, -32, -61,
    ];
    if percent > 99 {
        return Ok(GATE_ATTENUATION[25]);
    }
    Ok(GATE_ATTENUATION[(percent as f32 * 0.24) as usize])
}

pub fn gate_attenuation_values(percent: u8) -> Result<([u8; 4], i32), ProtocolError> {
    let attenuation_db = gate_attenuation_db_from_percent(percent)?;
    Ok((i8_as_f32_param_bytes(attenuation_db), attenuation_db as i32))
}

pub fn indexed_time_param_value(option: TimeOption) -> [u8; 4] {
    u8_as_f32_param_bytes(option.index)
}

pub fn indexed_time_effect_value(option: TimeOption) -> i32 {
    option.index as i32
}

pub fn compressor_threshold_values(threshold_db: i8) -> Result<([u8; 4], i32), ProtocolError> {
    let threshold_db = validate_compressor_threshold_db(threshold_db)?;
    Ok((i8_as_f32_param_bytes(threshold_db), threshold_db as i32))
}

pub fn compressor_ratio_values(
    ratio: CompressorRatioOption,
) -> Result<([u8; 4], i32), ProtocolError> {
    let value = ratio.ratio_tenths as f32 / 10.0;
    Ok((f32_param_bytes(value), ratio.index as i32))
}

pub fn compressor_makeup_gain_values(gain_db: i8) -> Result<([u8; 4], i32), ProtocolError> {
    let gain_db = validate_compressor_makeup_gain_db(gain_db)?;
    Ok((i8_as_f32_param_bytes(gain_db), gain_db as i32))
}

pub fn de_esser_effect_value(amount_percent: u8) -> Result<i32, ProtocolError> {
    Ok(validate_percent(amount_percent)? as i32)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CommandIndex {
    current: u16,
}

impl CommandIndex {
    pub fn next_for(&mut self, command: ProtocolCommand) -> u16 {
        if command == ProtocolCommand::ResetCommandIndex {
            self.current = 0;
        } else {
            self.current = self.current.wrapping_add(1);
            if self.current == 0 {
                self.current = 1;
            }
        }

        self.current
    }
}

pub fn frame_request(
    command: ProtocolCommand,
    command_index: u16,
    body: &[u8],
) -> Result<Vec<u8>, ProtocolError> {
    let body_len =
        u16::try_from(body.len()).map_err(|_| ProtocolError::BodyTooLarge(body.len()))?;
    let mut frame = vec![0; FRAME_HEADER_LEN + body.len()];
    frame[0..4].copy_from_slice(&command.command_id()?.to_le_bytes());
    frame[4..6].copy_from_slice(&body_len.to_le_bytes());
    frame[6..8].copy_from_slice(&command_index.to_le_bytes());
    frame[FRAME_HEADER_LEN..].copy_from_slice(body);
    Ok(frame)
}

pub fn parse_response(
    command: ProtocolCommand,
    expected_index: u16,
    response: &[u8],
) -> Result<Vec<u8>, ProtocolError> {
    if response.len() < FRAME_HEADER_LEN {
        return Err(ProtocolError::MalformedResponse {
            reason: format!(
                "response shorter than {FRAME_HEADER_LEN}-byte header: {}",
                response.len()
            ),
        });
    }

    let command_id = read_u32(&response[0..4]);
    let expected_command_id = command.command_id()?;
    if command_id != expected_command_id {
        return Err(ProtocolError::UnexpectedCommandId {
            expected: expected_command_id,
            actual: command_id,
        });
    }

    let body_len = read_u16(&response[4..6]) as usize;
    let actual_index = read_u16(&response[6..8]);
    if actual_index != expected_index {
        return Err(ProtocolError::UnexpectedCommandIndex {
            expected: expected_index,
            actual: actual_index,
        });
    }

    let actual_body_len = response.len() - FRAME_HEADER_LEN;
    if actual_body_len != body_len {
        return Err(ProtocolError::UnexpectedBodyLength {
            expected: body_len,
            actual: actual_body_len,
        });
    }

    Ok(response[FRAME_HEADER_LEN..].to_vec())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonStateSnapshot {
    pub pressed_bits: u32,
    pub fader_volumes: [u8; 4],
    pub fader_mute_pressed: [bool; 4],
    pub encoders: [i8; 4],
}

impl ButtonStateSnapshot {
    pub fn fader_volume(&self, fader: FaderName) -> FaderVolume {
        FaderVolume::from_raw(self.fader_volumes[fader.index()])
    }

    pub fn fader_mute_pressed(&self, fader: FaderName) -> bool {
        self.fader_mute_pressed[fader.index()]
    }
}

pub fn parse_button_state_response(response: &[u8]) -> Result<ButtonStateSnapshot, ProtocolError> {
    if response.len() < 12 {
        return Err(ProtocolError::MalformedResponse {
            reason: format!(
                "button state response shorter than 12 bytes: {}",
                response.len()
            ),
        });
    }

    let pressed_bits = read_u32(&response[0..4]);
    let encoders = [
        response[4] as i8,
        response[5] as i8,
        response[6] as i8,
        response[7] as i8,
    ];
    let fader_volumes = [response[8], response[9], response[10], response[11]];
    let fader_mute_pressed = [4_u32, 9, 14, 19].map(|bit| pressed_bits & (1 << bit) != 0);

    Ok(ButtonStateSnapshot {
        pressed_bits,
        fader_volumes,
        fader_mute_pressed,
        encoders,
    })
}

/// Decode the fader assignment payload shape used by GoXLR Utility's
/// `SetFader` command.
///
/// This is intentionally a payload decoder, not a hardware "get assignment"
/// command. Upstream GoXLR Utility exposes fader assignment from its active
/// profile state and writes it with the `SetFader` command.
pub fn parse_fader_assignment_payload(response: &[u8]) -> Result<ChannelName, ProtocolError> {
    if response.len() < 4 {
        return Err(ProtocolError::MalformedResponse {
            reason: format!(
                "fader assignment payload shorter than 4 bytes: {}",
                response.len()
            ),
        });
    }

    Ok(channel_from_protocol_index(response[0]))
}

/// Decode GoXLR Utility profile mute button flags into the IPC mute state.
///
/// Upstream treats the blink flag as "muted to all" and the state flag as
/// "muted to X"; blink wins when both flags are set.
pub fn mute_state_from_profile_flags(muted_to_x: bool, muted_to_all: bool) -> FaderMuteState {
    if muted_to_all {
        return FaderMuteState::MutedToAll;
    }
    if muted_to_x {
        return FaderMuteState::MutedToX;
    }

    FaderMuteState::Unmuted
}

pub fn parse_mute_state_payload(response: &[u8]) -> Result<FaderMuteState, ProtocolError> {
    if response.len() < 2 {
        return Err(ProtocolError::MalformedResponse {
            reason: format!(
                "mute state payload shorter than 2 bytes: {}",
                response.len()
            ),
        });
    }

    Ok(mute_state_from_profile_flags(
        response[0] != 0,
        response[1] != 0,
    ))
}

pub fn mute_function_from_profile_index(index: u8) -> MuteFunction {
    match index {
        0 => MuteFunction::All,
        1 => MuteFunction::ToStream,
        2 => MuteFunction::ToVoiceChat,
        3 => MuteFunction::ToPhones,
        4 => MuteFunction::ToLineOut,
        5 => MuteFunction::ToStream2,
        6 => MuteFunction::ToStreams,
        _ => MuteFunction::Unknown,
    }
}

pub fn parse_mute_function_payload(response: &[u8]) -> Result<MuteFunction, ProtocolError> {
    let Some(index) = response.first() else {
        return Err(ProtocolError::MalformedResponse {
            reason: "mute function payload is empty".to_string(),
        });
    };

    Ok(mute_function_from_profile_index(*index))
}

pub fn parse_firmware_response(response: &[u8]) -> Result<VersionNumber, ProtocolError> {
    if response.len() < 8 {
        return Err(ProtocolError::MalformedResponse {
            reason: format!("firmware response shorter than 8 bytes: {}", response.len()),
        });
    }

    let firmware_packed = read_u32(&response[0..4]);
    let firmware_build = read_u32(&response[4..8]);
    Ok(VersionNumber {
        major: firmware_packed >> 12,
        minor: (firmware_packed >> 8) & 0x0f,
        patch: Some(firmware_packed & 0xff),
        build: Some(firmware_build),
    })
}

pub fn parse_serial_response(response: &[u8]) -> Result<(String, String), ProtocolError> {
    if response.len() < 24 {
        return Err(ProtocolError::MalformedResponse {
            reason: format!("serial response shorter than 24 bytes: {}", response.len()),
        });
    }

    let serial_number = null_terminated_ascii(&response[..24]);
    let manufacture_date = null_terminated_ascii(&response[24..]);
    Ok((serial_number, manufacture_date))
}

pub fn volume_from_raw(raw: u8) -> FaderVolume {
    FaderVolume::from_raw(raw)
}

pub fn raw_volume_from_percent(percent: u8) -> Result<u8, ProtocolError> {
    Ok(FaderVolume::from_percent(percent)?.raw)
}

pub fn channel_protocol_index(channel: ChannelName) -> Result<u8, ProtocolError> {
    let index = match channel {
        ChannelName::Mic => 0,
        ChannelName::LineIn => 1,
        ChannelName::Console => 2,
        ChannelName::System => 3,
        ChannelName::Game => 4,
        ChannelName::Chat => 5,
        ChannelName::Sample => 6,
        ChannelName::Music => 7,
        ChannelName::Headphones => 8,
        ChannelName::MicMonitor => 9,
        ChannelName::LineOut => 10,
        ChannelName::Unknown => return Err(ProtocolError::UnsupportedChannel(channel)),
    };

    Ok(index)
}

pub fn channel_from_protocol_index(index: u8) -> ChannelName {
    match index {
        0 => ChannelName::Mic,
        1 => ChannelName::LineIn,
        2 => ChannelName::Console,
        3 => ChannelName::System,
        4 => ChannelName::Game,
        5 => ChannelName::Chat,
        6 => ChannelName::Sample,
        7 => ChannelName::Music,
        8 => ChannelName::Headphones,
        9 => ChannelName::MicMonitor,
        10 => ChannelName::LineOut,
        _ => ChannelName::Unknown,
    }
}

fn null_terminated_ascii(value: &[u8]) -> String {
    let len = value
        .iter()
        .position(|item| *item == 0)
        .unwrap_or(value.len());
    String::from_utf8_lossy(&value[..len]).to_string()
}

fn read_u16(value: &[u8]) -> u16 {
    u16::from_le_bytes([value[0], value[1]])
}

fn read_u32(value: &[u8]) -> u32 {
    u32::from_le_bytes([value[0], value[1], value[2], value[3]])
}

#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("protocol body is too large: {0} bytes")]
    BodyTooLarge(usize),

    #[error("unsupported channel for protocol command: {0}")]
    UnsupportedChannel(ChannelName),

    #[error("unsupported routing output for this payload: {0}")]
    UnsupportedRoutingOutput(RoutingOutput),

    #[error("unsupported EQ band for this command family: {0:?}")]
    UnsupportedEqBand(EqBandId),

    #[error("unexpected command id: expected {expected:#x}, received {actual:#x}")]
    UnexpectedCommandId { expected: u32, actual: u32 },

    #[error("unexpected command index: expected {expected}, received {actual}")]
    UnexpectedCommandIndex { expected: u16, actual: u16 },

    #[error("unexpected body length: expected {expected}, received {actual}")]
    UnexpectedBodyLength { expected: usize, actual: usize },

    #[error("malformed response: {reason}")]
    MalformedResponse { reason: String },

    #[error(transparent)]
    Model(#[from] goxlr_model::ModelError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_product_ids_to_models() {
        assert_eq!(model_from_product_id(GOXLR_PRODUCT_ID), DeviceModel::GoXlr);
        assert_eq!(
            model_from_product_id(GOXLR_MINI_PRODUCT_ID),
            DeviceModel::GoXlrMini
        );
        assert_eq!(model_from_product_id(0), DeviceModel::Unknown);
    }

    #[test]
    fn frames_command_requests() {
        let frame = frame_request(
            ProtocolCommand::SetChannelVolume(ChannelName::Music),
            7,
            &[128],
        )
        .unwrap();

        assert_eq!(&frame[0..4], &((0x806_u32 << 12) | 7).to_le_bytes());
        assert_eq!(&frame[4..6], &1_u16.to_le_bytes());
        assert_eq!(&frame[6..8], &7_u16.to_le_bytes());
        assert_eq!(frame[16], 128);
    }

    #[test]
    fn advances_and_resets_command_indexes() {
        let mut index = CommandIndex::default();

        assert_eq!(index.next_for(ProtocolCommand::GetButtonStates), 1);
        assert_eq!(index.next_for(ProtocolCommand::GetButtonStates), 2);
        assert_eq!(index.next_for(ProtocolCommand::ResetCommandIndex), 0);
        assert_eq!(index.next_for(ProtocolCommand::GetButtonStates), 1);
    }

    #[test]
    fn validates_response_index_and_length() {
        let mut response = frame_request(ProtocolCommand::GetButtonStates, 2, &[0; 12]).unwrap();

        assert!(parse_response(ProtocolCommand::GetButtonStates, 2, &response).is_ok());

        response[6..8].copy_from_slice(&3_u16.to_le_bytes());
        assert!(matches!(
            parse_response(ProtocolCommand::GetButtonStates, 2, &response),
            Err(ProtocolError::UnexpectedCommandIndex { .. })
        ));
    }

    #[test]
    fn rejects_malformed_response() {
        assert!(matches!(
            parse_response(ProtocolCommand::GetButtonStates, 1, &[0; 8]),
            Err(ProtocolError::MalformedResponse { .. })
        ));
    }

    #[test]
    fn parses_button_state_response() {
        let pressed = (1_u32 << 4) | (1_u32 << 19);
        let mut response = vec![0; 12];
        response[0..4].copy_from_slice(&pressed.to_le_bytes());
        response[8..12].copy_from_slice(&[10, 20, 30, 40]);

        let state = parse_button_state_response(&response).unwrap();

        assert_eq!(state.fader_volumes, [10, 20, 30, 40]);
        assert_eq!(state.fader_mute_pressed, [true, false, false, true]);
    }

    #[test]
    fn parses_fader_assignment_payload() {
        assert_eq!(
            parse_fader_assignment_payload(&[5, 0, 0, 0]).unwrap(),
            ChannelName::Chat
        );
        assert_eq!(
            parse_fader_assignment_payload(&[255, 0, 0, 0]).unwrap(),
            ChannelName::Unknown
        );
    }

    #[test]
    fn parses_mute_state_payload() {
        assert_eq!(
            parse_mute_state_payload(&[0, 0]).unwrap(),
            FaderMuteState::Unmuted
        );
        assert_eq!(
            parse_mute_state_payload(&[1, 0]).unwrap(),
            FaderMuteState::MutedToX
        );
        assert_eq!(
            parse_mute_state_payload(&[0, 1]).unwrap(),
            FaderMuteState::MutedToAll
        );
        assert_eq!(
            parse_mute_state_payload(&[1, 1]).unwrap(),
            FaderMuteState::MutedToAll
        );
    }

    #[test]
    fn parses_mute_function_payload() {
        assert_eq!(
            parse_mute_function_payload(&[0]).unwrap(),
            MuteFunction::All
        );
        assert_eq!(
            parse_mute_function_payload(&[2]).unwrap(),
            MuteFunction::ToVoiceChat
        );
        assert_eq!(
            parse_mute_function_payload(&[6]).unwrap(),
            MuteFunction::ToStreams
        );
        assert_eq!(
            parse_mute_function_payload(&[99]).unwrap(),
            MuteFunction::Unknown
        );
    }

    #[test]
    fn rejects_malformed_mixer_payloads() {
        assert!(matches!(
            parse_fader_assignment_payload(&[0, 0, 0]),
            Err(ProtocolError::MalformedResponse { .. })
        ));
        assert!(matches!(
            parse_mute_state_payload(&[0]),
            Err(ProtocolError::MalformedResponse { .. })
        ));
        assert!(matches!(
            parse_mute_function_payload(&[]),
            Err(ProtocolError::MalformedResponse { .. })
        ));
    }

    #[test]
    fn parses_firmware_and_serial_responses() {
        let packed = (1_u32 << 12) | (3_u32 << 8) | 40;
        let firmware =
            parse_firmware_response(&[packed.to_le_bytes(), 12_u32.to_le_bytes()].concat())
                .unwrap();
        assert_eq!(firmware.to_string(), "1.3.40.12");

        let mut serial = [0_u8; 32];
        serial[..8].copy_from_slice(b"ABC12345");
        serial[24..28].copy_from_slice(b"2024");
        let (number, date) = parse_serial_response(&serial).unwrap();

        assert_eq!(number, "ABC12345");
        assert_eq!(date, "2024");
    }

    #[test]
    fn converts_volume_values() {
        assert_eq!(volume_from_raw(128).percent, 50);
        assert_eq!(raw_volume_from_percent(50).unwrap(), 128);
        assert!(raw_volume_from_percent(101).is_err());
    }

    #[test]
    fn encodes_routing_payloads() {
        let routing = RoutingState {
            routes: vec![
                RoutingRouteState {
                    route: RoutingRoute {
                        input: RoutingInput::Microphone,
                        output: RoutingOutput::Headphones,
                    },
                    enabled: true,
                },
                RoutingRouteState {
                    route: RoutingRoute {
                        input: RoutingInput::Microphone,
                        output: RoutingOutput::BroadcastMix,
                    },
                    enabled: true,
                },
            ],
        };

        let payloads = build_routing_payloads(&routing, RoutingInput::Microphone, false).unwrap();

        assert_eq!(payloads[0].input, ProtocolRoutingInput::MicrophoneLeft);
        assert_eq!(payloads[1].input, ProtocolRoutingInput::MicrophoneRight);
        assert_eq!(payloads[0].body.len(), 22);
        assert_eq!(payloads[0].body[1], 0x20);
        assert_eq!(payloads[1].body[3], 0x20);
        assert_eq!(payloads[0].body[5], 0x20);
        assert_eq!(payloads[1].body[7], 0x20);
    }

    #[test]
    fn decodes_routing_payloads_and_rejects_malformed_lengths() {
        let mut left = vec![0; 22];
        let mut right = vec![0; 22];
        left[1] = 0x20;
        right[3] = 0x20;

        let routes = parse_routing_payloads(RoutingInput::Music, &left, &right, false).unwrap();

        assert!(
            routes
                .iter()
                .find(|state| state.route.output == RoutingOutput::Headphones)
                .unwrap()
                .enabled
        );
        assert!(matches!(
            parse_routing_payloads(RoutingInput::Music, &left[..21], &right, false),
            Err(ProtocolError::MalformedResponse { .. })
        ));
    }

    #[test]
    fn rejects_stream_mix_two_without_mix2_payload_support() {
        let routing = RoutingState {
            routes: vec![RoutingRouteState {
                route: RoutingRoute {
                    input: RoutingInput::Music,
                    output: RoutingOutput::StreamMix2,
                },
                enabled: true,
            }],
        };

        assert!(matches!(
            build_routing_payloads(&routing, RoutingInput::Music, false),
            Err(ProtocolError::UnsupportedRoutingOutput(
                RoutingOutput::StreamMix2
            ))
        ));
    }

    #[test]
    fn encodes_microphone_gain_and_type_params() {
        let params = microphone_gain_params(MicrophoneType::Condenser, 42).unwrap();
        let data = encode_microphone_params(&params);

        assert_eq!(
            &data[0..4],
            &(MicrophoneParamKey::MicType as u32).to_le_bytes()
        );
        assert_eq!(&data[4..8], &[1, 0, 0, 0]);
        assert_eq!(
            &data[8..12],
            &(MicrophoneParamKey::CondenserGain as u32).to_le_bytes()
        );
        assert_eq!(&data[12..16], &[0, 0, 42, 0]);
        assert!(microphone_gain_params(MicrophoneType::Dynamic, 73).is_err());
    }

    #[test]
    fn encodes_equalizer_values() {
        assert_eq!(eq_frequency_effect_value(10000), 135);
        assert_eq!(eq_gain_effect_value(-3).unwrap(), -3);
        assert!(eq_gain_effect_value(10).is_err());

        let value = eq_mini_frequency_param_value(900);
        assert_eq!(f32::from_le_bytes(value), 90.0);
        assert_eq!(
            f32::from_le_bytes(eq_mini_gain_param_value(4).unwrap()),
            4.0
        );
    }

    #[test]
    fn encodes_gate_values() {
        assert_eq!(gate_threshold_values(-30).unwrap().1, -30);
        assert_eq!(gate_attenuation_db_from_percent(100).unwrap(), -61);
        assert_eq!(gate_attenuation_db_from_percent(0).unwrap(), -6);
        assert!(gate_threshold_values(-60).is_err());
        assert!(gate_attenuation_db_from_percent(101).is_err());
    }

    #[test]
    fn encodes_compressor_and_de_esser_values() {
        let ratio = goxlr_model::compressor_ratio_option(9).unwrap();
        let (ratio_param, ratio_effect) = compressor_ratio_values(ratio).unwrap();

        assert_eq!(f32::from_le_bytes(ratio_param), 4.0);
        assert_eq!(ratio_effect, 9);
        assert_eq!(compressor_threshold_values(-18).unwrap().1, -18);
        assert_eq!(compressor_makeup_gain_values(6).unwrap().1, 6);
        assert_eq!(de_esser_effect_value(35).unwrap(), 35);
        assert!(compressor_threshold_values(-41).is_err());
        assert!(compressor_makeup_gain_values(25).is_err());
        assert!(de_esser_effect_value(101).is_err());
    }
}
