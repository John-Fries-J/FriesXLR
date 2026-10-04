//! Read-only Windows discovery through the official TC-Helicon TUSBAUDIO API.
//!
//! This module intentionally does not send vendor control requests. It only enumerates
//! driver-reported devices and reads their public properties.
//!
//! The TUSBAUDIO function names, registry CLSID, and default DLL location are adapted
//! from the MIT-licensed GoXLR Utility project. See `NOTICE.md` for attribution.

use crate::{DeviceConnectionInfo, DeviceError, DeviceProvider, DiscoverySnapshot, DriverInfo};
use goxlr_model::{DeviceIdentity, VersionNumber};
use goxlr_protocol::model_from_product_id;
use libloading::Library;
use std::ffi::CStr;
use std::path::PathBuf;
use tracing::{debug, warn};
use winreg::enums::HKEY_CLASSES_ROOT;
use winreg::RegKey;

type EnumerateDevices = unsafe extern "C" fn() -> u32;
type GetDriverInfo = unsafe extern "C" fn(*mut RawDriverInfo) -> u32;
type GetDeviceCount = unsafe extern "C" fn() -> u32;
type OpenDeviceByIndex = unsafe extern "C" fn(u32, *mut u32) -> u32;
type GetDeviceInstanceIdString = unsafe extern "C" fn(u32, *mut u16, u32) -> u32;
type GetDeviceProperties = unsafe extern "C" fn(u32, *mut RawProperties) -> u32;
type StatusCodeString = unsafe extern "C" fn(u32) -> *const i8;
type CloseDevice = unsafe extern "C" fn(u32) -> u32;

#[derive(Debug)]
pub struct WindowsDeviceProvider {
    api: Result<TusbApi, String>,
}

impl Default for WindowsDeviceProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowsDeviceProvider {
    pub fn new() -> Self {
        let api = TusbApi::load().map_err(|error| error.to_string());
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
                model: model_from_product_id(product_id),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_null_terminated_wide_strings() {
        let input = ['G' as u16, 'o' as u16, 'X' as u16, 0, 'x' as u16];

        assert_eq!(wide_to_string(&input), Some("GoX".to_string()));
    }
}
