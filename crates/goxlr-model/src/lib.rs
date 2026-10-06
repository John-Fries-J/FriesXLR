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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RoutingInput {
    Microphone,
    Chat,
    Music,
    Game,
    Console,
    LineIn,
    System,
    #[serde(rename = "sample")]
    Sample,
}

impl RoutingInput {
    pub const ALL: [Self; 8] = [
        Self::Microphone,
        Self::Chat,
        Self::Music,
        Self::Game,
        Self::Console,
        Self::LineIn,
        Self::System,
        Self::Sample,
    ];
}

impl Display for RoutingInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RoutingInput::Microphone => f.write_str("Mic"),
            RoutingInput::Chat => f.write_str("Chat"),
            RoutingInput::Music => f.write_str("Music"),
            RoutingInput::Game => f.write_str("Game"),
            RoutingInput::Console => f.write_str("Console"),
            RoutingInput::LineIn => f.write_str("Line In"),
            RoutingInput::System => f.write_str("System"),
            RoutingInput::Sample => f.write_str("Sample"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RoutingOutput {
    Headphones,
    BroadcastMix,
    ChatMic,
    Sampler,
    LineOut,
    StreamMix2,
}

impl RoutingOutput {
    pub const BASE: [Self; 5] = [
        Self::Headphones,
        Self::BroadcastMix,
        Self::ChatMic,
        Self::Sampler,
        Self::LineOut,
    ];

    pub const WITH_STREAM_MIX_2: [Self; 6] = [
        Self::Headphones,
        Self::BroadcastMix,
        Self::ChatMic,
        Self::Sampler,
        Self::LineOut,
        Self::StreamMix2,
    ];
}

impl Display for RoutingOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RoutingOutput::Headphones => f.write_str("Headphones"),
            RoutingOutput::BroadcastMix => f.write_str("Broadcast Mix"),
            RoutingOutput::ChatMic => f.write_str("Chat Mic"),
            RoutingOutput::Sampler => f.write_str("Sampler"),
            RoutingOutput::LineOut => f.write_str("Line Out"),
            RoutingOutput::StreamMix2 => f.write_str("Stream Mix 2"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutingRoute {
    pub input: RoutingInput,
    pub output: RoutingOutput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutingRouteState {
    pub route: RoutingRoute,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutingState {
    pub routes: Vec<RoutingRouteState>,
}

impl RoutingState {
    pub fn default_for_outputs(outputs: &[RoutingOutput]) -> Self {
        let defaults = [
            (RoutingInput::Microphone, RoutingOutput::BroadcastMix),
            (RoutingInput::Microphone, RoutingOutput::ChatMic),
            (RoutingInput::Microphone, RoutingOutput::Headphones),
            (RoutingInput::Microphone, RoutingOutput::Sampler),
            (RoutingInput::Microphone, RoutingOutput::LineOut),
            (RoutingInput::LineIn, RoutingOutput::Headphones),
            (RoutingInput::System, RoutingOutput::Headphones),
            (RoutingInput::System, RoutingOutput::LineOut),
            (RoutingInput::Game, RoutingOutput::BroadcastMix),
            (RoutingInput::Game, RoutingOutput::Headphones),
            (RoutingInput::Game, RoutingOutput::LineOut),
            (RoutingInput::Chat, RoutingOutput::Headphones),
            (RoutingInput::Chat, RoutingOutput::LineOut),
            (RoutingInput::Music, RoutingOutput::BroadcastMix),
            (RoutingInput::Music, RoutingOutput::Headphones),
            (RoutingInput::Music, RoutingOutput::LineOut),
            (RoutingInput::Sample, RoutingOutput::BroadcastMix),
            (RoutingInput::Sample, RoutingOutput::ChatMic),
            (RoutingInput::Sample, RoutingOutput::Headphones),
            (RoutingInput::Sample, RoutingOutput::LineOut),
            (RoutingInput::Console, RoutingOutput::BroadcastMix),
            (RoutingInput::Console, RoutingOutput::Headphones),
            (RoutingInput::Console, RoutingOutput::LineOut),
        ];

        let routes = supported_routes_for_outputs(outputs)
            .into_iter()
            .map(|route| RoutingRouteState {
                enabled: defaults.contains(&(route.input, route.output)),
                route,
            })
            .collect();

        Self { routes }
    }

    pub fn is_enabled(&self, route: RoutingRoute) -> Option<bool> {
        self.routes
            .iter()
            .find(|state| state.route == route)
            .map(|state| state.enabled)
    }

    pub fn set_enabled(&mut self, route: RoutingRoute, enabled: bool) -> Result<(), ModelError> {
        let Some(state) = self.routes.iter_mut().find(|state| state.route == route) else {
            return Err(ModelError::UnsupportedRoute(route));
        };

        state.enabled = enabled;
        Ok(())
    }
}

pub fn supported_routes_for_outputs(outputs: &[RoutingOutput]) -> Vec<RoutingRoute> {
    let mut routes = Vec::new();
    for input in RoutingInput::ALL {
        for output in outputs {
            let route = RoutingRoute {
                input,
                output: *output,
            };
            if is_supported_route(route) {
                routes.push(route);
            }
        }
    }
    routes
}

pub fn is_supported_route(route: RoutingRoute) -> bool {
    !(route.input == RoutingInput::Chat && route.output == RoutingOutput::ChatMic)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MicrophoneType {
    Dynamic,
    Condenser,
    #[serde(rename = "jack")]
    Jack,
}

impl MicrophoneType {
    pub const ALL: [Self; 3] = [Self::Dynamic, Self::Condenser, Self::Jack];

    pub fn has_phantom_power(self) -> bool {
        matches!(self, Self::Condenser)
    }
}

impl Display for MicrophoneType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MicrophoneType::Dynamic => f.write_str("Dynamic"),
            MicrophoneType::Condenser => f.write_str("Condenser"),
            MicrophoneType::Jack => f.write_str("3.5mm"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EqBandId {
    Eq31Hz,
    Eq63Hz,
    Eq125Hz,
    Eq250Hz,
    Eq500Hz,
    Eq1KHz,
    Eq2KHz,
    Eq4KHz,
    Eq8KHz,
    Eq16KHz,
    MiniEq90Hz,
    MiniEq250Hz,
    MiniEq500Hz,
    MiniEq1KHz,
    MiniEq3KHz,
    MiniEq8KHz,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EqBandCapability {
    pub id: EqBandId,
    pub label: String,
    pub default_frequency_tenths_hz: u32,
    pub min_frequency_tenths_hz: u32,
    pub max_frequency_tenths_hz: u32,
    pub frequency_step_tenths_hz: u32,
    pub min_gain_db: i8,
    pub max_gain_db: i8,
    pub configurable_frequency: bool,
    pub configurable_width: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueRangeI16 {
    pub min: i16,
    pub max: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueRangeU16 {
    pub min: u16,
    pub max: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeOption {
    pub index: u8,
    pub millis: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressorRatioOption {
    pub index: u8,
    pub ratio_tenths: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GateControl {
    Enabled,
    Threshold,
    Attenuation,
    Attack,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CompressorControl {
    Threshold,
    Ratio,
    Attack,
    Release,
    MakeupGain,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicrophoneGainState {
    pub microphone_type: MicrophoneType,
    pub hardware_db: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicrophoneSetupState {
    pub microphone_type: MicrophoneType,
    pub gains: Vec<MicrophoneGainState>,
    pub phantom_power_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EqBandState {
    pub id: EqBandId,
    pub frequency_tenths_hz: u32,
    pub gain_db: i8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EqualizerState {
    pub bands: Vec<EqBandState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoiseGateState {
    pub enabled: bool,
    pub threshold_db: i8,
    pub attenuation_percent: u8,
    pub attack: TimeOption,
    pub release: TimeOption,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressorState {
    pub threshold_db: i8,
    pub ratio: CompressorRatioOption,
    pub attack: TimeOption,
    pub release: TimeOption,
    pub makeup_gain_db: i8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeEsserState {
    pub amount_percent: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicrophoneState {
    pub setup: MicrophoneSetupState,
    pub equalizer: EqualizerState,
    pub gate: NoiseGateState,
    pub compressor: CompressorState,
    pub de_esser: DeEsserState,
}

impl MicrophoneState {
    pub fn default_for_model(model: DeviceModel) -> Self {
        let bands = match model {
            DeviceModel::GoXlr => full_eq_band_capabilities(),
            DeviceModel::GoXlrMini | DeviceModel::Unknown => mini_eq_band_capabilities(),
        }
        .into_iter()
        .map(|band| EqBandState {
            id: band.id,
            frequency_tenths_hz: band.default_frequency_tenths_hz,
            gain_db: 0,
        })
        .collect();

        Self {
            setup: MicrophoneSetupState {
                microphone_type: MicrophoneType::Dynamic,
                gains: MicrophoneType::ALL
                    .into_iter()
                    .map(|microphone_type| MicrophoneGainState {
                        microphone_type,
                        hardware_db: 0,
                    })
                    .collect(),
                phantom_power_enabled: false,
            },
            equalizer: EqualizerState { bands },
            gate: NoiseGateState {
                enabled: false,
                threshold_db: -30,
                attenuation_percent: 100,
                attack: gate_time_option(0).expect("default gate attack index is valid"),
                release: gate_time_option(19).expect("default gate release index is valid"),
            },
            compressor: CompressorState {
                threshold_db: 0,
                ratio: compressor_ratio_option(9).expect("default compressor ratio index is valid"),
                attack: compressor_attack_option(1)
                    .expect("default compressor attack index is valid"),
                release: compressor_release_option(9)
                    .expect("default compressor release index is valid"),
                makeup_gain_db: 0,
            },
            de_esser: DeEsserState { amount_percent: 0 },
        }
    }

    pub fn gain_for_mut(
        &mut self,
        microphone_type: MicrophoneType,
    ) -> Option<&mut MicrophoneGainState> {
        self.setup
            .gains
            .iter_mut()
            .find(|gain| gain.microphone_type == microphone_type)
    }

    pub fn eq_band_mut(&mut self, band_id: EqBandId) -> Option<&mut EqBandState> {
        self.equalizer
            .bands
            .iter_mut()
            .find(|band| band.id == band_id)
    }
}

pub const GAIN_DB_RANGE: ValueRangeU16 = ValueRangeU16 { min: 0, max: 72 };
pub const EQ_GAIN_RANGE: ValueRangeI16 = ValueRangeI16 { min: -9, max: 9 };
pub const GATE_THRESHOLD_RANGE: ValueRangeI16 = ValueRangeI16 { min: -59, max: 0 };
pub const COMPRESSOR_THRESHOLD_RANGE: ValueRangeI16 = ValueRangeI16 { min: -40, max: 0 };
pub const COMPRESSOR_MAKEUP_GAIN_RANGE: ValueRangeI16 = ValueRangeI16 { min: -6, max: 24 };

pub const GATE_TIMES: [u16; 46] = [
    10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200, 250,
    300, 350, 400, 450, 500, 550, 600, 650, 700, 750, 800, 850, 900, 950, 1000, 1100, 1200, 1300,
    1400, 1500, 1600, 1700, 1800, 1900, 2000,
];

pub const COMPRESSOR_ATTACK_TIMES: [u16; 20] = [
    0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 16, 18, 20, 23, 26, 30, 35, 40,
];

pub const COMPRESSOR_RELEASE_TIMES: [u16; 20] = [
    0, 15, 25, 35, 45, 55, 65, 75, 85, 100, 115, 140, 170, 230, 340, 680, 1000, 1500, 2000, 3000,
];

pub const COMPRESSOR_RATIO_TENTHS: [u16; 15] = [
    10, 11, 12, 14, 16, 18, 20, 25, 32, 40, 56, 80, 160, 320, 640,
];

pub fn gate_time_option(index: u8) -> Option<TimeOption> {
    GATE_TIMES.get(index as usize).map(|millis| TimeOption {
        index,
        millis: *millis,
    })
}

pub fn gate_time_options() -> Vec<TimeOption> {
    GATE_TIMES
        .iter()
        .enumerate()
        .map(|(index, millis)| TimeOption {
            index: index as u8,
            millis: *millis,
        })
        .collect()
}

pub fn compressor_attack_option(index: u8) -> Option<TimeOption> {
    COMPRESSOR_ATTACK_TIMES
        .get(index as usize)
        .map(|millis| TimeOption {
            index,
            millis: *millis,
        })
}

pub fn compressor_attack_options() -> Vec<TimeOption> {
    COMPRESSOR_ATTACK_TIMES
        .iter()
        .enumerate()
        .map(|(index, millis)| TimeOption {
            index: index as u8,
            millis: *millis,
        })
        .collect()
}

pub fn compressor_release_option(index: u8) -> Option<TimeOption> {
    COMPRESSOR_RELEASE_TIMES
        .get(index as usize)
        .map(|millis| TimeOption {
            index,
            millis: *millis,
        })
}

pub fn compressor_release_options() -> Vec<TimeOption> {
    COMPRESSOR_RELEASE_TIMES
        .iter()
        .enumerate()
        .map(|(index, millis)| TimeOption {
            index: index as u8,
            millis: *millis,
        })
        .collect()
}

pub fn compressor_ratio_option(index: u8) -> Option<CompressorRatioOption> {
    COMPRESSOR_RATIO_TENTHS
        .get(index as usize)
        .map(|ratio_tenths| CompressorRatioOption {
            index,
            ratio_tenths: *ratio_tenths,
        })
}

pub fn compressor_ratio_options() -> Vec<CompressorRatioOption> {
    COMPRESSOR_RATIO_TENTHS
        .iter()
        .enumerate()
        .map(|(index, ratio_tenths)| CompressorRatioOption {
            index: index as u8,
            ratio_tenths: *ratio_tenths,
        })
        .collect()
}

pub fn full_eq_band_capabilities() -> Vec<EqBandCapability> {
    vec![
        eq_band(EqBandId::Eq31Hz, "31 Hz", 315, 300, 3000, 5),
        eq_band(EqBandId::Eq63Hz, "63 Hz", 630, 300, 3000, 5),
        eq_band(EqBandId::Eq125Hz, "125 Hz", 1250, 300, 3000, 5),
        eq_band(EqBandId::Eq250Hz, "250 Hz", 2500, 300, 3000, 5),
        eq_band(EqBandId::Eq500Hz, "500 Hz", 5000, 3000, 20000, 1000),
        eq_band(EqBandId::Eq1KHz, "1 kHz", 10000, 3000, 20000, 1000),
        eq_band(EqBandId::Eq2KHz, "2 kHz", 20000, 3000, 20000, 1000),
        eq_band(EqBandId::Eq4KHz, "4 kHz", 40000, 20000, 180000, 1000),
        eq_band(EqBandId::Eq8KHz, "8 kHz", 80000, 20000, 180000, 1000),
        eq_band(EqBandId::Eq16KHz, "16 kHz", 160000, 20000, 180000, 1000),
    ]
}

pub fn mini_eq_band_capabilities() -> Vec<EqBandCapability> {
    vec![
        eq_band(EqBandId::MiniEq90Hz, "90 Hz", 900, 300, 900, 10),
        eq_band(EqBandId::MiniEq250Hz, "250 Hz", 1600, 1000, 3000, 10),
        eq_band(EqBandId::MiniEq500Hz, "500 Hz", 4800, 3100, 8000, 10),
        eq_band(EqBandId::MiniEq1KHz, "1 kHz", 15000, 8000, 25000, 10),
        eq_band(EqBandId::MiniEq3KHz, "3 kHz", 45000, 26000, 50000, 10),
        eq_band(EqBandId::MiniEq8KHz, "8 kHz", 78000, 51000, 180000, 10),
    ]
}

fn eq_band(
    id: EqBandId,
    label: &str,
    default_frequency_tenths_hz: u32,
    min_frequency_tenths_hz: u32,
    max_frequency_tenths_hz: u32,
    frequency_step_tenths_hz: u32,
) -> EqBandCapability {
    EqBandCapability {
        id,
        label: label.to_string(),
        default_frequency_tenths_hz,
        min_frequency_tenths_hz,
        max_frequency_tenths_hz,
        frequency_step_tenths_hz,
        min_gain_db: EQ_GAIN_RANGE.min as i8,
        max_gain_db: EQ_GAIN_RANGE.max as i8,
        configurable_frequency: true,
        configurable_width: false,
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
    pub readable_routing: bool,
    pub writable_routing: bool,
    pub supported_routing_inputs: Vec<RoutingInput>,
    pub supported_routing_outputs: Vec<RoutingOutput>,
    pub supported_routes: Vec<RoutingRoute>,
    pub readable_microphone: bool,
    pub writable_microphone: bool,
    pub supported_microphone_types: Vec<MicrophoneType>,
    pub microphone_gain_range_db: Option<ValueRangeU16>,
    pub phantom_power_supported: bool,
    pub eq_bands: Vec<EqBandCapability>,
    pub gate_controls: Vec<GateControl>,
    pub gate_threshold_range_db: Option<ValueRangeI16>,
    pub gate_attenuation_range_percent: Option<ValueRangeU16>,
    pub gate_time_options: Vec<TimeOption>,
    pub compressor_controls: Vec<CompressorControl>,
    pub compressor_threshold_range_db: Option<ValueRangeI16>,
    pub compressor_makeup_gain_range_db: Option<ValueRangeI16>,
    pub compressor_ratio_options: Vec<CompressorRatioOption>,
    pub compressor_attack_options: Vec<TimeOption>,
    pub compressor_release_options: Vec<TimeOption>,
    pub de_esser_supported: bool,
    pub de_esser_range_percent: Option<ValueRangeU16>,
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
            readable_routing: false,
            writable_routing: false,
            supported_routing_inputs: Vec::new(),
            supported_routing_outputs: Vec::new(),
            supported_routes: Vec::new(),
            readable_microphone: false,
            writable_microphone: false,
            supported_microphone_types: Vec::new(),
            microphone_gain_range_db: None,
            phantom_power_supported: false,
            eq_bands: Vec::new(),
            gate_controls: Vec::new(),
            gate_threshold_range_db: None,
            gate_attenuation_range_percent: None,
            gate_time_options: Vec::new(),
            compressor_controls: Vec::new(),
            compressor_threshold_range_db: None,
            compressor_makeup_gain_range_db: None,
            compressor_ratio_options: Vec::new(),
            compressor_attack_options: Vec::new(),
            compressor_release_options: Vec::new(),
            de_esser_supported: false,
            de_esser_range_percent: None,
        }
    }

    pub fn mock() -> Self {
        let routing_outputs = RoutingOutput::BASE.to_vec();
        Self {
            readable_fader_assignments: true,
            readable_fader_volumes: true,
            readable_fader_mute_state: true,
            readable_fader_button_state: true,
            writable_fader_volumes: true,
            writable_fader_mute_state: true,
            writable_fader_assignments: true,
            supported_assignment_channels: ChannelName::ASSIGNABLE.to_vec(),
            readable_routing: true,
            writable_routing: true,
            supported_routing_inputs: RoutingInput::ALL.to_vec(),
            supported_routes: supported_routes_for_outputs(&routing_outputs),
            supported_routing_outputs: routing_outputs,
            readable_microphone: true,
            writable_microphone: true,
            supported_microphone_types: MicrophoneType::ALL.to_vec(),
            microphone_gain_range_db: Some(GAIN_DB_RANGE),
            phantom_power_supported: true,
            eq_bands: mini_eq_band_capabilities(),
            gate_controls: vec![
                GateControl::Enabled,
                GateControl::Threshold,
                GateControl::Attenuation,
                GateControl::Attack,
                GateControl::Release,
            ],
            gate_threshold_range_db: Some(GATE_THRESHOLD_RANGE),
            gate_attenuation_range_percent: Some(ValueRangeU16 { min: 0, max: 100 }),
            gate_time_options: gate_time_options(),
            compressor_controls: vec![
                CompressorControl::Threshold,
                CompressorControl::Ratio,
                CompressorControl::Attack,
                CompressorControl::Release,
                CompressorControl::MakeupGain,
            ],
            compressor_threshold_range_db: Some(COMPRESSOR_THRESHOLD_RANGE),
            compressor_makeup_gain_range_db: Some(COMPRESSOR_MAKEUP_GAIN_RANGE),
            compressor_ratio_options: compressor_ratio_options(),
            compressor_attack_options: compressor_attack_options(),
            compressor_release_options: compressor_release_options(),
            de_esser_supported: true,
            de_esser_range_percent: Some(ValueRangeU16 { min: 0, max: 100 }),
        }
    }

    pub fn physical_read_only(model: DeviceModel) -> Self {
        let routing_outputs = RoutingOutput::BASE.to_vec();
        let eq_bands = match model {
            DeviceModel::GoXlr => full_eq_band_capabilities(),
            DeviceModel::GoXlrMini | DeviceModel::Unknown => mini_eq_band_capabilities(),
        };

        Self {
            readable_fader_assignments: false,
            readable_fader_volumes: true,
            readable_fader_mute_state: false,
            readable_fader_button_state: true,
            writable_fader_volumes: false,
            writable_fader_mute_state: false,
            writable_fader_assignments: false,
            supported_assignment_channels: ChannelName::ASSIGNABLE.to_vec(),
            readable_routing: false,
            writable_routing: false,
            supported_routing_inputs: RoutingInput::ALL.to_vec(),
            supported_routes: supported_routes_for_outputs(&routing_outputs),
            supported_routing_outputs: routing_outputs,
            readable_microphone: false,
            writable_microphone: false,
            supported_microphone_types: MicrophoneType::ALL.to_vec(),
            microphone_gain_range_db: Some(GAIN_DB_RANGE),
            phantom_power_supported: true,
            eq_bands,
            gate_controls: vec![
                GateControl::Enabled,
                GateControl::Threshold,
                GateControl::Attenuation,
                GateControl::Attack,
                GateControl::Release,
            ],
            gate_threshold_range_db: Some(GATE_THRESHOLD_RANGE),
            gate_attenuation_range_percent: Some(ValueRangeU16 { min: 0, max: 100 }),
            gate_time_options: gate_time_options(),
            compressor_controls: vec![
                CompressorControl::Threshold,
                CompressorControl::Ratio,
                CompressorControl::Attack,
                CompressorControl::Release,
                CompressorControl::MakeupGain,
            ],
            compressor_threshold_range_db: Some(COMPRESSOR_THRESHOLD_RANGE),
            compressor_makeup_gain_range_db: Some(COMPRESSOR_MAKEUP_GAIN_RANGE),
            compressor_ratio_options: compressor_ratio_options(),
            compressor_attack_options: compressor_attack_options(),
            compressor_release_options: compressor_release_options(),
            de_esser_supported: true,
            de_esser_range_percent: Some(ValueRangeU16 { min: 0, max: 100 }),
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
    pub routing: Option<RoutingState>,
    pub microphone: Option<MicrophoneState>,
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

    #[error("unsupported routing path: {0:?}")]
    UnsupportedRoute(RoutingRoute),

    #[error("invalid microphone gain {0}; expected 0..=72 dB")]
    InvalidMicrophoneGain(u16),

    #[error("invalid EQ gain {0}; expected -9..=9 dB")]
    InvalidEqGain(i8),

    #[error("invalid EQ frequency {frequency_tenths_hz}; expected {min_tenths_hz}..={max_tenths_hz} tenths of Hz")]
    InvalidEqFrequency {
        frequency_tenths_hz: u32,
        min_tenths_hz: u32,
        max_tenths_hz: u32,
    },

    #[error("invalid percent {0}; expected 0..=100")]
    InvalidPercent(u8),

    #[error("invalid gate threshold {0}; expected -59..=0 dB")]
    InvalidGateThreshold(i8),

    #[error("invalid compressor threshold {0}; expected -40..=0 dB")]
    InvalidCompressorThreshold(i8),

    #[error("invalid compressor makeup gain {0}; expected -6..=24 dB")]
    InvalidCompressorMakeupGain(i8),

    #[error("invalid indexed option {index}; expected 0..={max}")]
    InvalidIndexedOption { index: u8, max: u8 },
}

pub fn validate_volume(volume: u16) -> Result<u8, ModelError> {
    u8::try_from(volume).map_err(|_| ModelError::InvalidFaderVolume(volume))
}

pub fn validate_microphone_gain_db(gain: u16) -> Result<u16, ModelError> {
    if gain > GAIN_DB_RANGE.max {
        return Err(ModelError::InvalidMicrophoneGain(gain));
    }
    Ok(gain)
}

pub fn validate_eq_gain_db(gain: i8) -> Result<i8, ModelError> {
    if !(EQ_GAIN_RANGE.min as i8..=EQ_GAIN_RANGE.max as i8).contains(&gain) {
        return Err(ModelError::InvalidEqGain(gain));
    }
    Ok(gain)
}

pub fn validate_eq_frequency(
    capability: &EqBandCapability,
    frequency_tenths_hz: u32,
) -> Result<u32, ModelError> {
    if !(capability.min_frequency_tenths_hz..=capability.max_frequency_tenths_hz)
        .contains(&frequency_tenths_hz)
    {
        return Err(ModelError::InvalidEqFrequency {
            frequency_tenths_hz,
            min_tenths_hz: capability.min_frequency_tenths_hz,
            max_tenths_hz: capability.max_frequency_tenths_hz,
        });
    }
    Ok(frequency_tenths_hz)
}

pub fn validate_percent(percent: u8) -> Result<u8, ModelError> {
    if percent > 100 {
        return Err(ModelError::InvalidPercent(percent));
    }
    Ok(percent)
}

pub fn validate_gate_threshold_db(threshold: i8) -> Result<i8, ModelError> {
    if !(GATE_THRESHOLD_RANGE.min as i8..=GATE_THRESHOLD_RANGE.max as i8).contains(&threshold) {
        return Err(ModelError::InvalidGateThreshold(threshold));
    }
    Ok(threshold)
}

pub fn validate_compressor_threshold_db(threshold: i8) -> Result<i8, ModelError> {
    if !(COMPRESSOR_THRESHOLD_RANGE.min as i8..=COMPRESSOR_THRESHOLD_RANGE.max as i8)
        .contains(&threshold)
    {
        return Err(ModelError::InvalidCompressorThreshold(threshold));
    }
    Ok(threshold)
}

pub fn validate_compressor_makeup_gain_db(gain: i8) -> Result<i8, ModelError> {
    if !(COMPRESSOR_MAKEUP_GAIN_RANGE.min as i8..=COMPRESSOR_MAKEUP_GAIN_RANGE.max as i8)
        .contains(&gain)
    {
        return Err(ModelError::InvalidCompressorMakeupGain(gain));
    }
    Ok(gain)
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

    #[test]
    fn builds_supported_routing_without_chat_to_chat_mic() {
        let routes = supported_routes_for_outputs(&RoutingOutput::BASE);

        assert!(routes.contains(&RoutingRoute {
            input: RoutingInput::Microphone,
            output: RoutingOutput::ChatMic,
        }));
        assert!(!routes.contains(&RoutingRoute {
            input: RoutingInput::Chat,
            output: RoutingOutput::ChatMic,
        }));
    }

    #[test]
    fn default_routing_matches_upstream_profile_shape() {
        let routing = RoutingState::default_for_outputs(&RoutingOutput::BASE);

        assert_eq!(
            routing.is_enabled(RoutingRoute {
                input: RoutingInput::Microphone,
                output: RoutingOutput::BroadcastMix,
            }),
            Some(true)
        );
        assert_eq!(
            routing.is_enabled(RoutingRoute {
                input: RoutingInput::Chat,
                output: RoutingOutput::BroadcastMix,
            }),
            Some(false)
        );
        assert_eq!(
            routing.is_enabled(RoutingRoute {
                input: RoutingInput::Chat,
                output: RoutingOutput::ChatMic,
            }),
            None
        );
    }

    #[test]
    fn exposes_model_specific_equalizer_bands() {
        assert_eq!(full_eq_band_capabilities().len(), 10);
        assert_eq!(mini_eq_band_capabilities().len(), 6);
        assert_eq!(
            full_eq_band_capabilities()[0].default_frequency_tenths_hz,
            315
        );
    }

    #[test]
    fn validates_microphone_ranges() {
        assert!(validate_microphone_gain_db(72).is_ok());
        assert!(validate_microphone_gain_db(73).is_err());
        assert!(validate_eq_gain_db(-9).is_ok());
        assert!(validate_eq_gain_db(10).is_err());
        assert!(validate_gate_threshold_db(-59).is_ok());
        assert!(validate_gate_threshold_db(-60).is_err());
        assert!(validate_compressor_threshold_db(-40).is_ok());
        assert!(validate_compressor_threshold_db(-41).is_err());
        assert!(validate_compressor_makeup_gain_db(24).is_ok());
        assert!(validate_compressor_makeup_gain_db(25).is_err());
    }

    #[test]
    fn maps_indexed_audio_options() {
        assert_eq!(gate_time_option(0).unwrap().millis, 10);
        assert_eq!(gate_time_option(45).unwrap().millis, 2000);
        assert!(gate_time_option(46).is_none());
        assert_eq!(compressor_ratio_option(9).unwrap().ratio_tenths, 40);
        assert_eq!(compressor_attack_option(1).unwrap().millis, 2);
        assert_eq!(compressor_release_option(9).unwrap().millis, 100);
    }
}
