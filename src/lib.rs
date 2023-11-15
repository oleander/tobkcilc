#![no_std]

extern crate alloc;
extern crate anyhow;
extern crate hashbrown;
extern crate lazy_static;
extern crate linked_list_allocator;
extern crate log;
extern crate spin;

mod constants;
mod impls;
mod keyboard;
mod types;

use crate::constants::*;
use crate::types::*;
use anyhow::Result;
use core::option::Option::{None, Some};
use core::result::Result::Ok;
use esp_idf_hal::gpio::Pull;
use esp_idf_hal::gpio::*;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_sys::gpio_install_isr_service;
use esp_idf_sys::gpio_isr_handler_add;
use esp_idf_sys::gpio_set_pull_mode;
use log::*;

extern "C" fn gpio_interrupt_handler() {
  // Your interrupt handling code here
}

use core::ffi::c_void;

extern "C" fn cb(_x: *mut c_void) {

}

fn callback() {
  info!("Callback called");
}

#[allow(dead_code)]
#[no_mangle]
extern "C" fn app_main() {
  esp_idf_sys::link_patches();
  esp_idf_svc::log::EspLogger::initialize_default();

  info!("Starting up...");

  let peripherals = Peripherals::take().unwrap();
  let pins = peripherals.pins;

  info!("Setting up pin 0");
  let mut pin0 = pins.gpio0;

  info!("Setting up pin 0");
  let mut input = PinDriver::input(&mut pin0).unwrap();

  info!("Installing ISR service");
  let result = unsafe { input.subscribe(callback); };
  info!("Result: {:?}", result);

  info!("Subscribed to pin interrupt");
  input.set_pull(Pull::Up).unwrap();

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
    },
  };

  Ok(())
}
