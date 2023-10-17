#![feature(error_in_core)]
#![warn(unused_must_use)]
#![no_main]


extern crate log;

mod keyboard;
use keyboard::Keyboard;
use log::{info, warn};
use esp_idf_hal::delay::Ets::delay_ms;

#[no_mangle]
fn app_main() {
  esp_idf_sys::link_patches();
  esp_idf_svc::log::EspLogger::initialize_default();

  info!("Starting ESP32 keyboard driver");

  let mut keyboard = Keyboard::new();

  info!("Waiting for keyboard to connect");

  loop {
    if keyboard.connected() {
      info!("Sending awake command");
      keyboard.send_media_key(keyboard::media_keys::EMAIL_READER);
      delay_ms(5 * 1000 * 60); // 5 minutes
    } else {
      info!("Waiting for keyboard to connect");
      delay_ms(5000);
    }
  }
}
