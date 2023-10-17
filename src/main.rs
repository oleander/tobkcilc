#![feature(error_in_core)]
#![warn(unused_must_use)]
#![no_main]


extern crate log;

mod keyboard;
use keyboard::Keyboard;
use keyboard::media_keys::*;
use log::{info, warn};
use esp_idf_hal::delay::Ets;

#[no_mangle]
fn app_main() {
  esp_idf_sys::link_patches();
  esp_idf_svc::log::EspLogger::initialize_default();

  let mut keyboard = Keyboard::new();

  info!("Starting clickbot loop");

  loop {
    if keyboard.connected() {
      info!("Sending awake command");
      keyboard.send_media_key(EMAIL_READER);
      Ets::delay_ms(5 * 1000 * 60); // 5 minutes
    } else {
      info!("Waiting for keyboard to connect");
      Ets::delay_ms(5000);
    }
  }
}
