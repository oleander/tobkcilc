#![no_std]

extern crate linked_list_allocator;
extern crate lazy_static;
extern crate hashbrown;
extern crate anyhow;
extern crate alloc;
extern crate spin;
extern crate log;

mod keyboard;
mod constants;
mod types;
mod impls;

use crate::constants::*;
use crate::types::*;
use anyhow::Result;
use core::option::Option::{None, Some};
use core::result::Result::Ok;
use esp_idf_hal::gpio::*;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_sys::gpio_set_pull_mode;
use log::*;

#[allow(dead_code)]
#[no_mangle]
extern "C" fn app_main() {
  esp_idf_sys::link_patches();
  esp_idf_svc::log::EspLogger::initialize_default();

  info!("Starting up...");

  // let keyboard = KEYBOARD.lock();

  // esp_idf_hal::delay::Ets::delay_ms(2000);

  // while !keyboard.connected() {
  //   esp_idf_hal::delay::Ets::delay_ms(500);
  //   warn!("Keyboard not connected");
  // }

  // for _ in 0..3 {
  //   keyboard.send_media_key([0x00, 0x00]);
  //   esp_idf_hal::delay::Ets::delay_ms(500);
  // }
  // info!("Keyboard connected");

  // esp_idf_hal::gpio::
  // Set pin 3 as INPUT, PULLUP
  let peripherals = Peripherals::take().unwrap();
  let pins = peripherals.pins;

  // let mut pin = pins.gpio2.into_input()?;

  let pin = pins.gpio2;
  let pull = Pull::Up;
  unsafe { gpio_set_pull_mode(pin.pin(), pull.into()) };

  // let mut pin_x = PinDriver::input(pin)?; pin_x.set_pull(Pull::Up);


  loop {
    info!("Waiting for button click");
    esp_idf_hal::delay::Ets::delay_ms(1000);
  }
}

#[no_mangle]
extern "C" fn rust_handle_button_click(index: u8) {
  let Some(curr_state) = InputState::from(index) else {
    return error!("Invalid button index: {}", index);
  };

  if let Err(e) = handle_button_click(curr_state) {
    error!("Error handling button click: {:?}", e);
  }
}

impl From<InvalidButtonTransitionError> for anyhow::Error {
  fn from(e: InvalidButtonTransitionError) -> Self {
    anyhow::anyhow!("Invalid button transition: {:?}", e)
  }
}

fn handle_button_click(curr_state: InputState) -> Result<()> {
  info!("Handling button click: {:?}", curr_state);

  let mut state_guard = CURRENT_INPUT_STATE.lock();
  debug!("Current state: {:?}", *state_guard);

  let (event, new_state) = state_guard.transition_to(curr_state)?;
  *state_guard = new_state;

  info!("New state: {:?}", *state_guard);
  info!("New event: {:?}", event);

  match event {
    Some(BluetoothEvent::MediaControlKey(key)) => {
      KEYBOARD.lock().send_media_key(key.into());
    },
    Some(BluetoothEvent::Letter(letter)) => {
      KEYBOARD.lock().send_shortcut(letter);
    },
    None => {
      warn!("No event for button click: {:?}", curr_state);
    }
  };

  Ok(())
}
