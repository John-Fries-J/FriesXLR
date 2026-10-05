//! Protocol identifiers and packet helpers used by FriesXLR.
//!
//! These constants and packet shapes are adapted from the MIT-licensed GoXLR Utility
//! project:
//! https://github.com/GoXLR-on-Linux/goxlr-utility
//! See `NOTICE.md` for attribution.

use goxlr_model::{ChannelName, DeviceModel, FaderName, FaderVolume, VersionNumber};

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
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareInfoCommand {
    FirmwareVersion = 0,
    SerialNumber = 1,
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
}
