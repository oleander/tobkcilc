#![no_main]

extern crate anyhow;
extern crate hashbrown;
extern crate lazy_static;
extern crate log;

mod constants;
mod impls;
mod keyboard;
mod types;

use crate::keyboard::Keyboard;
use crate::types::*;
use esp_idf_hal::task;

use core::option::Option::Some;
use core::result::Result::Ok;
use esp_idf_hal::delay;
use esp_idf_hal::gpio::Pull;
use esp_idf_hal::gpio::*;
use esp_idf_hal::prelude::Peripherals;
use log::*;
use std::time::Duration;

macro_rules! pin {
  ($pin:expr) => {
    let pid_id = $pin.pin() as u32;
    let mut input = PinDriver::input(&mut $pin).unwrap();
    input.set_interrupt_type(InterruptType::LowLevel).unwrap();
    let handle = task::current().unwrap();
    input.set_pull(Pull::Up).unwrap();
    input.enable_interrupt().unwrap();
    let _subscription = unsafe {
      input
        .subscribe(move || {
          task::notify(handle, pid_id);
        })
        .unwrap()
    };
  };
}

#[no_mangle]
fn app_main() {
  esp_idf_sys::link_patches();
  esp_idf_svc::log::EspLogger::initialize_default();

  info!("Starting up...");

  info!("Ensure iPhone is connected");
  let keyboard = Keyboard::new();
  while !keyboard.connected() {
    info!("Waiting for keyboard to connect...");
    delay::Ets::delay_ms(200);
  }

  info!("iPhone connected");
  let peripherals = Peripherals::take().unwrap();
  let duration = Some(Duration::from_millis(200));
  let mut pins = peripherals.pins;

  info!("Initializing pins ...");
  pin!(pins.gpio1);
  pin!(pins.gpio3);
  pin!(pins.gpio4);
  pin!(pins.gpio5);
  pin!(pins.gpio6);
  pin!(pins.gpio7);
  pin!(pins.gpio9);
  pin!(pins.gpio10);

  let mut prev_state = InputState::Undefined;

  info!("Entering loop");
  loop {
    unsafe {
      esp_idf_sys::esp_task_wdt_reset();
    }

    let Some(pin_id) = task::wait_notification(duration) else {
      continue;
    };

    let Some(curr_state) = InputState::from(pin_id) else {
      warn!("Invalid button index: {}", pin_id);
      continue;
    };

    let (event, new_state) = match prev_state.transition_to(curr_state) {
      Ok(success) => success,
      Err(err) => {
        error!("Invalid transition: {:?}", err);
        continue;
      },
    };

    prev_state = new_state;

    info!("Curr state: {:?}", curr_state);
    info!("Prev state: {:?}", prev_state);
    info!("New state: {:?}", new_state);
    info!("New event: {:?}", event);

    if !keyboard.connected() {
      warn!("Phone is not connected");
      continue;
    }

    match event {
      Some(BluetoothEvent::MediaControlKey(key)) => {
        keyboard.send_media_key(key.into());
      },
      Some(BluetoothEvent::Letter(letter)) => {
        keyboard.send_shortcut(letter);
      },
      None => {
        warn!("No event for button click: {:?}", curr_state);
      },
    };
  }
}
