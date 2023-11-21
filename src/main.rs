#![feature(error_in_core)]
#![warn(unused_must_use)]
#![no_main]

extern crate log;
extern crate embassy_time;

mod keyboard;

use keyboard::Keyboard;
use log::{info, warn};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
async fn app_main() {
  esp_idf_sys::link_patches();
  esp_idf_svc::log::EspLogger::initialize_default();
  esp_idf_svc::timer::embassy_time::driver::link();

  info!("Starting clickbot loop");
  let mut keyboard = Keyboard::new();

  info!("Waiting for client ...");
  while !keyboard.connected() {
    print!(".");
    keyboard.delay_secs(5).await;
  }

  info!("\nClient connected, will send a keypress every 5 seconds");

  while keyboard.connected() {
    print!(".");
    keyboard.write("a").await;
    keyboard.delay_secs(5).await;
  }

  warn!("\nClient disconnected, rebooting in 5 seconds ...");
  keyboard.delay_secs(5).await;
  unsafe {
    esp_idf_sys::esp_restart();
  };
}
