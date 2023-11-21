#![feature(error_in_core)]
#![warn(unused_must_use)]
#![no_main]

extern crate embassy_time;
extern crate log;

mod keyboard;

use keyboard::Keyboard;
use log::{debug, info, warn};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
async fn app_main() {
  esp_idf_sys::link_patches();
  esp_idf_svc::log::EspLogger::initialize_default();
  esp_idf_svc::timer::embassy_time::driver::link();
  log::set_max_level(log::LevelFilter::Info);
  // log::set_max_level(log::LevelFilter::Trace);

  info!("Starting clickbot loop");
  let mut keyboard = keyboard::Keyboard::new();

  info!("Waiting for client to connect ...");
  while !keyboard.connected() {
    keyboard.delay_secs(1).await;
  }

  info!("Sending test keypresses");
  for _ in 0..5 {
    info!("Sending init keypress");
    keyboard.send_init().await;
  }

  info!("Client connected ...");
  info!("Sending test keypresses");

  while keyboard.connected() {
    info!("Sending awake keypress");
    keyboard.send_awake().await;
  }

  warn!("Client disconnected, will restart in 5 seconds");
  keyboard.delay_secs(5).await;
  unsafe { esp_idf_sys::esp_restart(); };
}
