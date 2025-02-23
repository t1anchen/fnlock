extern crate hidapi;

use hidapi::HidApi;

fn main() {
  println!("Printing all available hid devices:");

  match HidApi::new() {
    Ok(api) => {
      for device in api.device_list() {
        println!(
          "{:04x}:{:04x} -> {:?},{:?}",
          device.vendor_id(),
          device.product_id(),
          device.manufacturer_string().unwrap_or(""),
          device.product_string().unwrap_or("")
        );
      }
    }
    Err(e) => {
      eprintln!("Error: {}", e);
    }
  }
}
