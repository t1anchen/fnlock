// Borrowed from https://github.com/AnmSleepalone/setfnlock/blob/main/src-tauri/src/setfn.rs

use hidapi::{DeviceInfo as HidApiDeviceInfo, HidApi, HidDevice};
use log::{debug, error, info};

const K380_VID: u16 = 0x046d;
const K380_PID: u16 = 0xb342;
const TARGET_USAGE: u16 = 0x0001;
const TARGET_USAGE_PAGE: u16 = 0xff00;
const K380_SEQ_FKEYS_ON: [u8; 7] = [0x10, 0xff, 0x0b, 0x1e, 0x00, 0x00, 0x00];
const K380_SEQ_FKEYS_OFF: [u8; 7] = [0x10, 0xff, 0x0b, 0x1e, 0x01, 0x00, 0x00];

pub fn fnlock(opened_device: Option<HidDevice>, to_be_locked: bool) {
  if let Some(k380) = opened_device {
    if to_be_locked {
      match k380.write(&K380_SEQ_FKEYS_ON) {
        Ok(_) => info!("K380 Fn key has been locked"),
        Err(err) => error!("Unable to lock: {:?}", err),
      };
    } else {
      match k380.write(&K380_SEQ_FKEYS_OFF) {
        Ok(_) => info!("K380 Fn key has been unlocked"),
        Err(err) => error!("Unable to unlock: {:?}", err),
      };
    }
  }
}

#[derive(Debug)]
pub struct DeviceInfo {
  pub hid_device_info: HidApiDeviceInfo,
  pub product_name: String,
  pub vendor_name: String,
  pub product_id: u16,
  pub vendor_id: u16,
  pub usage: u16,
  pub usage_page: u16,
}

pub fn get_api_context() -> Option<HidApi> {
  match HidApi::new() {
    Ok(api) => Some(api),
    _ => None,
  }
}

pub fn list_devices(api: Option<&HidApi>) -> Vec<DeviceInfo> {
  let mut devices: Vec<DeviceInfo> = vec![];
  if let Some(_api) = api {
    for hid_device_info in _api.device_list() {
      debug!("hid_debug_info={:?}", hid_device_info);
      let device_info = DeviceInfo {
        hid_device_info: hid_device_info.clone(),
        product_name: hid_device_info
          .product_string()
          .map(|s| s.trim())
          .unwrap_or_default()
          .to_owned(),
        vendor_name: hid_device_info
          .manufacturer_string()
          .map(|s| s.trim())
          .unwrap_or_default()
          .to_owned(),
        product_id: hid_device_info.product_id(),
        vendor_id: hid_device_info.vendor_id(),
        usage: hid_device_info.usage(),
        usage_page: hid_device_info.usage_page(),
      };
      debug!("device_info={:?}", device_info);
      devices.push(device_info);
    }
  }
  devices
}

pub fn is_device_available(device: &DeviceInfo) -> bool {
  device.product_id == K380_PID
    && device.vendor_id == K380_VID
    && device.usage == TARGET_USAGE
    && device.usage_page == TARGET_USAGE_PAGE
}

pub fn find_device_from_deviceinfo(
  api: Option<&HidApi>,
  device_info: &DeviceInfo,
) -> Option<HidDevice> {
  api.and_then(|_api| match device_info.hid_device_info.open_device(_api) {
    Ok(device) => Some(device),
    _ => None,
  })
}

pub fn find_device() -> Option<HidDevice> {
  let mut result: Option<HidDevice> = None;
  let api = get_api_context();
  api.map(|_api| {
    for device_info in list_devices(Some(&_api)).iter() {
      if is_device_available(&device_info) {
        info!("target device found: {:?}", device_info);
        result = find_device_from_deviceinfo(Some(&_api), &device_info)
      }
    }
  });
  result
}
