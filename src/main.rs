#![feature(error_in_core)]
#![warn(unused_must_use)]
#![no_main]

extern crate log;

mod keyboard;

use esp_idf_hal::delay::Ets;
use keyboard::media_keys::*;
use keyboard::Keyboard;
use log::{info, warn};

#[no_mangle]
fn app_main() {
  esp_idf_sys::link_patches();
  esp_idf_svc::log::EspLogger::initialize_default();

  info!("Starting clickbot loop");
  let mut keyboard = Keyboard::new();

  info!("Running tests 10 times with 5 second delay");
  let all_events = vec![
    STOP,
    WWW_HOME,
    LOCAL_MACHINE_BROWSER,
    CALCULATOR,
    WWW_BOOKMARKS,
    WWW_SEARCH,
    WWW_STOP,
    WWW_BACK,
    CONSUMER_CONTROL_CONFIGURATION,
    EMAIL_READER,
  ];

  info!("Starting main clickbot loop");
  loop {
    if keyboard.connected() {
      info!("Sending awake command");
      // keyboard.send_media_key(WWW_HOME);

      keyboard.write("A");

      // for event in &all_events {
      //   info!("Sending event {:?}", event);
      //   keyboard.send_media_key(*event);
      //   Ets::delay_ms(5000);
      // }

      Ets::delay_ms(1 * 1000 * 60); // 5 minutes
      // Ets::delay_ms(5000);
    } else {
      info!("Waiting for keyboard to connect");
      Ets::delay_ms(5000);
    }
  }
}
