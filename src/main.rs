#![feature(error_in_core)]
#![warn(unused_must_use)]
#![no_main]

extern crate embassy_time;
extern crate log;

mod keyboard;

use std::sync::Arc;

use keyboard::Keyboard;
use log::{debug, info, warn};
use tokio::sync::Notify;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
async fn app_main() {
  esp_idf_sys::link_patches();
  esp_idf_svc::log::EspLogger::initialize_default();
  esp_idf_svc::timer::embassy_time::driver::link();
  log::set_max_level(log::LevelFilter::Info);

  let mut keyboard = keyboard::Keyboard::new();

  let notify = Arc::new(Notify::new());
  let notify_clone = notify.clone();

  keyboard.on_authentication_complete(move |conn| {
    info!("Terrain Command connected to {:?}", conn);
    notify_clone.notify_one();
  });

  info!("Waiting for iPhone to connect");
  notify.notified().await;
  info!("iPhone connected");

  info!("Connected to host");
  while keyboard.connected() {
    info!("Sending keypresses");
    keyboard.volume_down().await;
    keyboard.delay_secs(10).await;
  }

  warn!("Disconnected from host");
  warn!("Will restart in 5 seconds");
  keyboard.delay_secs(5).await;
  unsafe { esp_idf_sys::esp_restart(); }
}
