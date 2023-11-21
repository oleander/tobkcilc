#![feature(error_in_core)]
#![warn(unused_must_use)]
#![no_main]

extern crate log;
extern crate embassy_time;

mod keyboard;

use esp_idf_hal::delay::Ets;
use keyboard::Keyboard;
use log::{info, warn};

macro_rules! halt {
    ($($arg:tt)*) => ({
        warn!($($arg)*);
        warn!("Rebooting in 5 seconds...");
        esp_idf_hal::delay::Ets::delay_ms(5000);
        unsafe { esp_idf_sys::esp_restart(); };
    })
}

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
async fn app_main() {
  esp_idf_sys::link_patches();
  esp_idf_svc::log::EspLogger::initialize_default();
  esp_idf_svc::timer::embassy_time::driver::link();

  info!("Starting clickbot loop");
  let mut keyboard = Keyboard::new();

  info!("Running tests 10 times with 5 second delay");
  let mut connected = false;

  // if keyboard.connected() {
  //   for _ in 0..10 {
  //     keyboard.send_illumination_event();
  //     Ets::delay_ms(5000);
  //   }
  // }

  info!("Starting main clickbot loop");

  loop {
    if keyboard.connected() {
      connected = true;
      info!("Sending awake command");
      keyboard.write("a").await;
      info!("Waiting for keyboard to connect");
    } else if connected {
      halt!("Disconnected, will restart");
    } else {
      info!("Waiting for keyboard to connect");
      Ets::delay_ms(5000);
    }
  }
}
