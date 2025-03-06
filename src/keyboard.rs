// Borrowed from https://github.com/AnmSleepalone/setfnlock/blob/main/src-tauri/src/setfn.rs

use hidapi::{HidApi, HidDevice};
use log::{error, info};

const K380_VID: u16 = 0x046d;
const K380_PID: u16 = 0xb342;
const TARGET_USAGE: u16 = 0x0001;
const TARGET_USAGE_PAGE: u16 = 0xff00;
const K380_SEQ_FKEYS_ON: [u8; 7] = [0x10, 0xff, 0x0b, 0x1e, 0x00, 0x00, 0x00];
const K380_SEQ_FKEYS_OFF: [u8; 7] = [0x10, 0xff, 0x0b, 0x1e, 0x01, 0x00, 0x00];

pub fn fnlock(k380: HidDevice, to_be_locked: bool) {
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

pub fn find_device() -> Option<HidDevice> {
  match HidApi::new() {
    Ok(api) => {
      for device in api.device_list() {
        if device.vendor_id() == K380_VID && device.product_id() == K380_PID {
          if device.usage() == TARGET_USAGE
            && device.usage_page() == TARGET_USAGE_PAGE
          {
            match device.open_device(&api) {
              Ok(k380) => {
                return Some(k380);
              }
              Err(err) => {
                error!("Unable to open device: {:?}", err);
              }
            }
          }
        }
      }
      None
    }
    Err(err) => {
      eprintln!("Unable to use hidapi: {:?}", err);
      None
    }
  }
}
