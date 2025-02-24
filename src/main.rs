extern crate hidapi;

use hidapi::HidApi;

use clap::Parser;

// Borrowed from https://github.com/AnmSleepalone/setfnlock/blob/main/src-tauri/src/setfn.rs

const K380_VID: u16 = 0x046d;
const K380_PID: u16 = 0xb342;
const TARGET_USAGE: u16 = 0x0001;
const TARGET_USAGE_PAGE: u16 = 0xff00;
const K380_SEQ_FKEYS_ON: [u8; 7] = [0x10, 0xff, 0x0b, 0x1e, 0x00, 0x00, 0x00];
const K380_SEQ_FKEYS_OFF: [u8; 7] = [0x10, 0xff, 0x0b, 0x1e, 0x01, 0x00, 0x00];

fn api_main(opts: &Opts) {
  let to_be_locked = !opts.unlock;

  match HidApi::new() {
    Ok(api) => {
      for device in api.device_list() {
        if device.vendor_id() == K380_VID && device.product_id() == K380_PID {
          if device.usage() == TARGET_USAGE
            && device.usage_page() == TARGET_USAGE_PAGE
          {
            match device.open_device(&api) {
              Ok(k380) => {
                if to_be_locked {
                  match k380.write(&K380_SEQ_FKEYS_ON) {
                    Ok(_) => println!("K380 Fn key has been locked"),
                    Err(err) => eprintln!("Unable to lock: {:?}", err),
                  };
                } else {
                  match k380.write(&K380_SEQ_FKEYS_OFF) {
                    Ok(_) => println!("K380 Fn key has been unlocked"),
                    Err(err) => eprintln!("Unable to unlock: {:?}", err),
                  };
                }
              }
              Err(err) => eprintln!("Unable to open device: {:?}", err),
            }
          }
        }
      }
    }
    Err(err) => {
      eprintln!("Unable to use hidapi: {:?}", err);
    }
  }
}

#[derive(Parser, Debug)]
#[command(about)]
struct Opts {
  #[arg(long)]
  unlock: bool,
}

fn main() {
  let opts = Opts::parse();
  api_main(&opts);
}
