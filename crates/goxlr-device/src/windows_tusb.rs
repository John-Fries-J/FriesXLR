//! Windows discovery and basic read-only sessions through the official TC-Helicon
//! TUSBAUDIO API.
//!
//! The TUSBAUDIO function names, registry CLSID, default DLL location, command
//! activation sequence, and request framing are adapted from the MIT-licensed GoXLR
//! Utility project. See `NOTICE.md` for attribution.

use crate::{
    unavailable_faders, DeviceConnectionInfo, DeviceError, DeviceEvent, DeviceProvider,
    DeviceSession, DeviceSessionState, DiscoverySnapshot, DriverInfo, SessionGeneration,
};
use goxlr_model::{DeviceCapabilities, DeviceIdentity, FaderName, FaderState, VersionNumber};
use goxlr_protocol::{
    frame_request, parse_button_state_response, parse_firmware_response, parse_response,
    parse_serial_response, ButtonStateSnapshot, CommandIndex, HardwareInfoCommand, ProtocolCommand,
    MAX_RESPONSE_LEN,
};
use libloading::Library;
use std::collections::VecDeque;
use std::ffi::CStr;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tracing::{debug, info, trace, warn};
use winreg::enums::HKEY_CLASSES_ROOT;
use winreg::RegKey;

type EnumerateDevices = unsafe extern "C" fn() -> u32;
type GetDriverInfo = unsafe extern "C" fn(*mut RawDriverInfo) -> u32;
type GetDeviceCount = unsafe extern "C" fn() -> u32;
type OpenDeviceByIndex = unsafe extern "C" fn(u32, *mut u32) -> u32;
type GetDeviceInstanceIdString = unsafe extern "C" fn(u32, *mut u16, u32) -> u32;
type GetDeviceProperties = unsafe extern "C" fn(u32, *mut RawProperties) -> u32;
type VendorRequestOut =
    unsafe extern "C" fn(u32, u32, u32, u32, u32, u16, u16, *const u8, *mut u8, u32) -> u32;
type VendorRequestIn =
    unsafe extern "C" fn(u32, u32, u32, u32, u32, u16, u16, *mut u8, *mut u8, u32) -> u32;
type StatusCodeString = unsafe extern "C" fn(u32) -> *const i8;
type CloseDevice = unsafe extern "C" fn(u32) -> u32;

const MIXER_POLL_INTERVAL: Duration = Duration::from_millis(50);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(1);

#[derive(Debug)]
pub struct WindowsDeviceProvider {
    api: Result<Arc<TusbApi>, String>,
}

impl Default for WindowsDeviceProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowsDeviceProvider {
    pub fn new() -> Self {
        let api = TusbApi::load()
            .map(Arc::new)
            .map_err(|error| error.to_string());
        Self { api }
    }
}

impl DeviceProvider for WindowsDeviceProvider {
    fn discover(&self) -> Result<DiscoverySnapshot, DeviceError> {
        let api = self
            .api
            .as_ref()
            .map_err(|error| DeviceError::DriverUnavailable(error.clone()))?;
        api.discover()
    }

    fn open_session(
        &self,
        identity: &DeviceIdentity,
        generation: SessionGeneration,
    ) -> Result<Box<dyn DeviceSession>, DeviceError> {
        let api = self
            .api
            .as_ref()
            .map_err(|error| DeviceError::DriverUnavailable(error.clone()))?
            .clone();

        Ok(Box::new(TusbDeviceSession::open(
            api,
            identity.clone(),
            generation,
        )?))
    }

    fn driver_info(&self) -> DriverInfo {
        match &self.api {
            Ok(api) => DriverInfo {
                interface: Some("tusb".to_string()),
                version: api.driver_version(),
                available: true,
                last_error: None,
            },
            Err(error) => DriverInfo {
                interface: Some("tusb".to_string()),
                version: None,
                available: false,
                last_error: Some(error.clone()),
            },
        }
    }
}

#[derive(Debug)]
struct TusbApi {
    _library: Library,
    enumerate_devices: EnumerateDevices,
    get_driver_info: Option<GetDriverInfo>,
    get_device_count: GetDeviceCount,
    open_device_by_index: OpenDeviceByIndex,
    get_device_id_string: GetDeviceInstanceIdString,
    get_device_properties: GetDeviceProperties,
    vendor_request_out: VendorRequestOut,
    vendor_request_in: VendorRequestIn,
    status_code_string: StatusCodeString,
    close_device: CloseDevice,
}

impl TusbApi {
    fn load() -> Result<Self, DeviceError> {
        let path = locate_library();
        debug!(?path, "loading GoXLR TUSBAUDIO API");
        let library = unsafe { Library::new(path.as_str()) }
            .map_err(|error| DeviceError::DriverUnavailable(error.to_string()))?;

        let api = unsafe {
            Self {
                enumerate_devices: *library
                    .get::<EnumerateDevices>(b"TUSBAUDIO_EnumerateDevices")
                    .map_err(|error| DeviceError::DriverUnavailable(error.to_string()))?,
                get_driver_info: library
                    .get::<GetDriverInfo>(b"TUSBAUDIO_GetDriverInfo")
                    .map(|symbol| *symbol)
                    .ok(),
                get_device_count: *library
                    .get::<GetDeviceCount>(b"TUSBAUDIO_GetDeviceCount")
                    .map_err(|error| DeviceError::DriverUnavailable(error.to_string()))?,
                open_device_by_index: *library
                    .get::<OpenDeviceByIndex>(b"TUSBAUDIO_OpenDeviceByIndex")
                    .map_err(|error| DeviceError::DriverUnavailable(error.to_string()))?,
                get_device_id_string: *library
                    .get::<GetDeviceInstanceIdString>(b"TUSBAUDIO_GetDeviceInstanceIdString")
                    .map_err(|error| DeviceError::DriverUnavailable(error.to_string()))?,
                get_device_properties: *library
                    .get::<GetDeviceProperties>(b"TUSBAUDIO_GetDeviceProperties")
                    .map_err(|error| DeviceError::DriverUnavailable(error.to_string()))?,
                vendor_request_out: *library
                    .get::<VendorRequestOut>(b"TUSBAUDIO_ClassVendorRequestOut")
                    .map_err(|error| DeviceError::DriverUnavailable(error.to_string()))?,
                vendor_request_in: *library
                    .get::<VendorRequestIn>(b"TUSBAUDIO_ClassVendorRequestIn")
                    .map_err(|error| DeviceError::DriverUnavailable(error.to_string()))?,
                status_code_string: *library
                    .get::<StatusCodeString>(b"TUSBAUDIO_StatusCodeStringA")
                    .map_err(|error| DeviceError::DriverUnavailable(error.to_string()))?,
                close_device: *library
                    .get::<CloseDevice>(b"TUSBAUDIO_CloseDevice")
                    .map_err(|error| DeviceError::DriverUnavailable(error.to_string()))?,
                _library: library,
            }
        };

        Ok(api)
    }

    fn discover(&self) -> Result<DiscoverySnapshot, DeviceError> {
        unsafe {
            (self.enumerate_devices)();
        }

        let device_count = unsafe { (self.get_device_count)() };
        let mut devices = Vec::new();

        for index in 0..device_count {
            match self.device_at(index) {
                Ok(Some(device)) => devices.push(device),
                Ok(None) => {}
                Err(error) => warn!(index, %error, "failed to inspect GoXLR driver device"),
            }
        }

        Ok(DiscoverySnapshot { devices })
    }

    fn device_at(&self, index: u32) -> Result<Option<DeviceConnectionInfo>, DeviceError> {
        let handle = self.open_device(index)?;
        let _close = CloseHandleOnDrop {
            handle,
            close_device: self.close_device,
        };

        let identifier = self.device_identifier(handle)?;
        let properties = self.properties(handle)?;
        let vendor_id = properties.vendor_id as u16;
        let product_id = properties.product_id as u16;

        if !goxlr_protocol::is_known_goxlr_device(vendor_id, product_id) {
            return Ok(None);
        }

        let serial_number = wide_to_string(&properties.serial_number);
        let manufacturer_name = wide_to_string(&properties.manufacturer);
        let product_name = wide_to_string(&properties.model);

        Ok(Some(DeviceConnectionInfo {
            identity: DeviceIdentity {
                id: identifier,
                model: goxlr_protocol::model_from_product_id(product_id),
                vendor_id: Some(vendor_id),
                product_id: Some(product_id),
                manufacturer_name,
                product_name,
                serial_number,
                firmware_version: None,
                driver_interface: Some("tusb".to_string()),
                driver_version: self.driver_version(),
                is_mock: false,
            },
            read_only_state: None,
        }))
    }

    fn open_device_by_identifier(&self, identifier: &str) -> Result<u32, DeviceError> {
        unsafe {
            (self.enumerate_devices)();
        }

        let device_count = unsafe { (self.get_device_count)() };
        for index in 0..device_count {
            let handle = self.open_device(index)?;
            match self.device_identifier(handle) {
                Ok(candidate) if candidate == identifier => return Ok(handle),
                _ => {
                    let _ = unsafe { (self.close_device)(handle) };
                }
            }
        }

        Err(DeviceError::DeviceUnavailable(identifier.to_string()))
    }

    fn open_device(&self, index: u32) -> Result<u32, DeviceError> {
        let mut handle = 0_u32;
        let result = unsafe { (self.open_device_by_index)(index, &mut handle) };
        if result != 0 {
            return Err(DeviceError::Discovery(self.error_text(result)));
        }
        Ok(handle)
    }

    fn device_identifier(&self, handle: u32) -> Result<String, DeviceError> {
        let mut buffer = vec![0_u16; 256];
        let result = unsafe { (self.get_device_id_string)(handle, buffer.as_mut_ptr(), 256) };
        if result != 0 {
            return Err(DeviceError::Discovery(self.error_text(result)));
        }

        Ok(wide_to_string(&buffer).unwrap_or_else(|| format!("tusb:{handle}")))
    }

    fn properties(&self, handle: u32) -> Result<RawProperties, DeviceError> {
        let mut properties = RawProperties::default();
        let result = unsafe { (self.get_device_properties)(handle, &mut properties) };
        if result != 0 {
            return Err(DeviceError::Discovery(self.error_text(result)));
        }
        Ok(properties)
    }

    fn send_request(
        &self,
        handle: u32,
        request: u8,
        value: u16,
        index: u16,
        data: &[u8],
    ) -> Result<(), DeviceError> {
        let data_length = u16::try_from(data.len())
            .map_err(|_| DeviceError::Protocol(format!("request is too large: {}", data.len())))?;
        let mut bytes_written = [0_u8; 64];
        let result = unsafe {
            (self.vendor_request_out)(
                handle,
                1,
                0,
                request.into(),
                value.into(),
                index,
                data_length,
                data.as_ptr(),
                bytes_written.as_mut_ptr(),
                bytes_written.len() as u32,
            )
        };

        if result != 0 {
            return Err(self.control_error(result));
        }

        Ok(())
    }

    fn read_response(
        &self,
        handle: u32,
        request: u8,
        value: u16,
        index: u16,
        length: usize,
    ) -> Result<Vec<u8>, DeviceError> {
        let data_length = u16::try_from(length)
            .map_err(|_| DeviceError::Protocol(format!("read is too large: {length}")))?;
        let mut buffer = vec![0_u8; length];
        let mut returned = [0_u8; 64];
        let result = unsafe {
            (self.vendor_request_in)(
                handle,
                1,
                0,
                request.into(),
                value.into(),
                index,
                data_length,
                buffer.as_mut_ptr(),
                returned.as_mut_ptr(),
                returned.len() as u32,
            )
        };

        if result != 0 {
            return Err(self.control_error(result));
        }

        let read_len =
            u32::from_le_bytes([returned[0], returned[1], returned[2], returned[3]]) as usize;
        if read_len > buffer.len() {
            return Err(DeviceError::MalformedResponse(format!(
                "driver reported {read_len} bytes for {length}-byte buffer"
            )));
        }

        buffer.truncate(read_len);
        Ok(buffer)
    }

    fn close_handle(&self, handle: u32) {
        let result = unsafe { (self.close_device)(handle) };
        if result != 0 {
            warn!(error = %self.error_text(result), "failed to close GoXLR TUSBAUDIO handle");
        }
    }

    fn driver_version(&self) -> Option<VersionNumber> {
        let get_driver_info = self.get_driver_info?;
        let mut info = RawDriverInfo::default();
        let result = unsafe { get_driver_info(&mut info) };
        if result != 0 {
            warn!(error = %self.error_text(result), "failed to read GoXLR driver version");
            return None;
        }

        Some(VersionNumber {
            major: info.driver_major,
            minor: info.driver_minor,
            patch: Some(info.driver_patch),
            build: None,
        })
    }

    fn control_error(&self, code: u32) -> DeviceError {
        let text = self.error_text(code);
        if text == "TSTATUS_INVALID_HANDLE" {
            DeviceError::DeviceDisconnected
        } else {
            DeviceError::Protocol(text)
        }
    }

    fn error_text(&self, code: u32) -> String {
        let pointer = unsafe { (self.status_code_string)(code) };
        if pointer.is_null() {
            return format!("TUSBAUDIO status {code}");
        }

        unsafe { CStr::from_ptr(pointer) }
            .to_string_lossy()
            .to_string()
    }
}

struct CloseHandleOnDrop {
    handle: u32,
    close_device: CloseDevice,
}

impl Drop for CloseHandleOnDrop {
    fn drop(&mut self) {
        let _ = unsafe { (self.close_device)(self.handle) };
    }
}

#[derive(Debug)]
struct TusbDeviceSession {
    api: Arc<TusbApi>,
    handle: Option<u32>,
    identity: DeviceIdentity,
    generation: SessionGeneration,
    command_index: CommandIndex,
    last_button_state: ButtonStateSnapshot,
    pending_events: VecDeque<DeviceEvent>,
    last_poll: Instant,
    closed: bool,
}

impl TusbDeviceSession {
    fn open(
        api: Arc<TusbApi>,
        mut identity: DeviceIdentity,
        generation: SessionGeneration,
    ) -> Result<Self, DeviceError> {
        let handle = api.open_device_by_identifier(&identity.id)?;
        let mut session = Self {
            api,
            handle: Some(handle),
            identity: identity.clone(),
            generation,
            command_index: CommandIndex::default(),
            last_button_state: ButtonStateSnapshot {
                pressed_bits: 0,
                fader_volumes: [0; 4],
                fader_mute_pressed: [false; 4],
                encoders: [0; 4],
            },
            pending_events: VecDeque::new(),
            last_poll: Instant::now(),
            closed: false,
        };

        session.activate_vendor_pipe()?;
        match session.read_serial_number() {
            Ok(serial) if !serial.is_empty() => identity.serial_number = Some(serial),
            Ok(_) => {}
            Err(error) => {
                warn!(device_id = %identity.id, %error, "failed to read GoXLR serial number")
            }
        }

        match session.read_firmware_version() {
            Ok(version) => identity.firmware_version = Some(version),
            Err(error) => {
                warn!(device_id = %identity.id, %error, "failed to read GoXLR firmware version")
            }
        }

        session.identity = identity;
        session.last_button_state = session.read_button_state()?;

        info!(
            device_id = %session.identity.id,
            serial = ?session.identity.serial_number,
            model = %session.identity.model,
            session_generation = generation,
            "device session opened"
        );

        Ok(session)
    }

    fn activate_vendor_pipe(&mut self) -> Result<(), DeviceError> {
        let handle = self.handle()?;
        let _ = self.api.read_response(handle, 0, 0, 0, 24)?;
        self.api.send_request(handle, 1, 0, 0, &[])?;
        sleep(Duration::from_millis(20));
        let _ = self.api.read_response(handle, 3, 0, 0, MAX_RESPONSE_LEN)?;
        Ok(())
    }

    fn read_serial_number(&mut self) -> Result<String, DeviceError> {
        let response = self.request(
            ProtocolCommand::GetHardwareInfo(HardwareInfoCommand::SerialNumber),
            &[],
        )?;
        let (serial, _) = parse_serial_response(&response)?;
        Ok(serial)
    }

    fn read_firmware_version(&mut self) -> Result<VersionNumber, DeviceError> {
        let response = self.request(
            ProtocolCommand::GetHardwareInfo(HardwareInfoCommand::FirmwareVersion),
            &[],
        )?;
        Ok(parse_firmware_response(&response)?)
    }

    fn read_button_state(&mut self) -> Result<ButtonStateSnapshot, DeviceError> {
        let response = self.request(ProtocolCommand::GetButtonStates, &[])?;
        Ok(parse_button_state_response(&response)?)
    }

    fn request(&mut self, command: ProtocolCommand, body: &[u8]) -> Result<Vec<u8>, DeviceError> {
        self.perform_request(command, body, false)
    }

    fn perform_request(
        &mut self,
        command: ProtocolCommand,
        body: &[u8],
        retry: bool,
    ) -> Result<Vec<u8>, DeviceError> {
        let index = self.command_index.next_for(command);
        let frame = frame_request(command, index, body)?;

        trace!(
            device_id = %self.identity.id,
            operation = "request",
            command_id = ?command.command_id().ok(),
            command_index = index,
            session_generation = self.generation,
            "sending GoXLR command"
        );

        self.api.send_request(self.handle()?, 2, 0, 0, &frame)?;

        let deadline = Instant::now() + REQUEST_TIMEOUT;
        loop {
            match self
                .api
                .read_response(self.handle()?, 3, 0, 0, MAX_RESPONSE_LEN)
            {
                Ok(response) => match parse_response(command, index, &response) {
                    Ok(body) => return Ok(body),
                    Err(goxlr_protocol::ProtocolError::UnexpectedCommandIndex { .. }) if !retry => {
                        debug!(
                            device_id = %self.identity.id,
                            session_generation = self.generation,
                            "command index mismatch; resetting command index"
                        );
                        let _ =
                            self.perform_request(ProtocolCommand::ResetCommandIndex, &[], true)?;
                        return self.perform_request(command, body, true);
                    }
                    Err(error) => return Err(error.into()),
                },
                Err(DeviceError::DeviceDisconnected) => {
                    self.closed = true;
                    return Err(DeviceError::DeviceDisconnected);
                }
                Err(error) => {
                    if Instant::now() >= deadline {
                        return Err(DeviceError::Timeout(error.to_string()));
                    }
                    sleep(Duration::from_millis(10));
                }
            }
        }
    }

    fn enqueue_button_state_events(&mut self, next: &ButtonStateSnapshot) {
        let device_id = self.identity.id.clone();
        for fader in FaderName::ALL {
            let index = fader.index();
            if self.last_button_state.fader_volumes[index] != next.fader_volumes[index] {
                self.pending_events
                    .push_back(DeviceEvent::FaderVolumeChanged {
                        device_id: device_id.clone(),
                        generation: self.generation,
                        fader,
                        volume: next.fader_volume(fader),
                    });
            }

            if self.last_button_state.fader_mute_pressed[index] != next.fader_mute_pressed[index] {
                self.pending_events
                    .push_back(DeviceEvent::FaderMuteButtonChanged {
                        device_id: device_id.clone(),
                        generation: self.generation,
                        fader,
                        pressed: next.fader_mute_pressed(fader),
                    });
            }
        }

        self.last_button_state = next.clone();
    }

    fn state_from_button_state(&self) -> DeviceSessionState {
        DeviceSessionState {
            identity: self.identity.clone(),
            capabilities: DeviceCapabilities::physical_read_only(),
            faders: FaderName::ALL
                .into_iter()
                .map(|name| FaderState {
                    name,
                    assigned_channel: None,
                    volume: Some(self.last_button_state.fader_volume(name)),
                    mute_state: None,
                    mute_function: None,
                    muted: None,
                    mute_button_pressed: Some(self.last_button_state.fader_mute_pressed(name)),
                })
                .collect(),
        }
    }

    fn handle(&self) -> Result<u32, DeviceError> {
        self.handle.ok_or(DeviceError::SessionClosed)
    }
}

impl DeviceSession for TusbDeviceSession {
    fn generation(&self) -> SessionGeneration {
        self.generation
    }

    fn current_state(&self) -> Result<DeviceSessionState, DeviceError> {
        if self.closed {
            return Err(DeviceError::SessionClosed);
        }
        Ok(self.state_from_button_state())
    }

    fn poll_event(&mut self) -> Result<Option<DeviceEvent>, DeviceError> {
        if self.closed {
            return Ok(None);
        }

        if let Some(event) = self.pending_events.pop_front() {
            return Ok(Some(event));
        }

        if self.last_poll.elapsed() < MIXER_POLL_INTERVAL {
            return Ok(None);
        }
        self.last_poll = Instant::now();

        match self.read_button_state() {
            Ok(next) => {
                self.enqueue_button_state_events(&next);
                Ok(self.pending_events.pop_front())
            }
            Err(DeviceError::DeviceDisconnected) => {
                self.closed = true;
                Ok(Some(DeviceEvent::Disconnected {
                    device_id: self.identity.id.clone(),
                    generation: self.generation,
                }))
            }
            Err(error) => Err(error),
        }
    }

    fn close(&mut self) {
        if let Some(handle) = self.handle.take() {
            self.api.close_handle(handle);
        }
        self.closed = true;
    }
}

impl Drop for TusbDeviceSession {
    fn drop(&mut self) {
        self.close();
    }
}

#[repr(C)]
#[derive(Debug, Default)]
struct RawDriverInfo {
    api_major: u32,
    api_minor: u32,
    driver_major: u32,
    driver_minor: u32,
    driver_patch: u32,
    flags: u32,
}

#[repr(C)]
#[derive(Debug)]
struct RawProperties {
    vendor_id: i32,
    product_id: i32,
    revision_number: i32,
    serial_number: [u16; 128],
    manufacturer: [u16; 128],
    model: [u16; 128],
    unknown_number: i32,
    unknown_string: [u16; 128],
}

impl Default for RawProperties {
    fn default() -> Self {
        Self {
            vendor_id: 0,
            product_id: 0,
            revision_number: 0,
            serial_number: [0; 128],
            manufacturer: [0; 128],
            model: [0; 128],
            unknown_number: 0,
            unknown_string: [0; 128],
        }
    }
}

fn locate_library() -> String {
    let regpath = "CLSID\\{024D0372-641F-4B7B-8140-F4DFE458C982}\\InprocServer32\\";
    let classes_root = RegKey::predef(HKEY_CLASSES_ROOT);

    if let Ok(folder) = classes_root.open_subkey(regpath) {
        if let Ok(api) = folder.get_value::<String, &str>("") {
            if PathBuf::from(&api).exists() {
                return api;
            }
        }
    }

    "C:/Program Files/TC-HELICON/GoXLR_Audio_Driver/W10_x64/goxlr_audioapi_x64.dll".to_string()
}

fn wide_to_string(value: &[u16]) -> Option<String> {
    let len = value
        .iter()
        .position(|item| *item == 0)
        .unwrap_or(value.len());
    if len == 0 {
        return None;
    }

    Some(String::from_utf16_lossy(&value[..len]))
}

#[allow(dead_code)]
fn empty_session_state(identity: DeviceIdentity) -> DeviceSessionState {
    DeviceSessionState {
        identity,
        capabilities: DeviceCapabilities::unavailable(),
        faders: unavailable_faders(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_null_terminated_wide_strings() {
        let input = ['G' as u16, 'o' as u16, 'X' as u16, 0, 'x' as u16];

        assert_eq!(wide_to_string(&input), Some("GoX".to_string()));
    }
}
