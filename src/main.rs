// #![no_std]
#![no_main]

extern crate anyhow;
extern crate hashbrown;
extern crate lazy_static;
extern crate log;

mod constants;
mod impls;
// mod keyboard;
mod types;

use crate::constants::*;
use esp_idf_hal::task;
use crate::types::*;
use anyhow::Result;

use esp_idf_hal::prelude::Peripherals;
use core::option::Option::Some;
use core::result::Result::Ok;
use esp_idf_hal::gpio::Pull;
use esp_idf_hal::gpio::*;
use std::time::Duration;
use log::*;

macro_rules! pin {
  ($pin:expr) => {
    let pid_id = $pin.pin() as u32;
    let mut input = PinDriver::input(&mut $pin).unwrap();
    input.set_interrupt_type(InterruptType::LowLevel).unwrap();
    let handle = task::current().unwrap();
    input.set_pull(Pull::Up).unwrap();
    input.enable_interrupt().unwrap();

    info!("Installing ISR service");
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

  let duration = Some(Duration::from_millis(100));
  let peripherals = Peripherals::take().unwrap();
  let mut pins = peripherals.pins;

  info!("Initializing pins ...");
  // GPIO0 has an internal pull-up resistor and is typically used to determine the boot mode at reset, but it can be used as a general input if it is not pulled LOW during boot
  pin!(pins.gpio0);
  pin!(pins.gpio1);
  // Also used for boot mode, similar caution as GPIO0
  // pin!(pins.gpio2); 
  pin!(pins.gpio3);
  pin!(pins.gpio4);
  pin!(pins.gpio5);
  // pin!(pins.gpio6);
  // pin!(pins.gpio7);
  pin!(pins.gpio8);
  pin!(pins.gpio9);
  // pin!(pins.gpio10);
  // pin!(pins.gpio18);
  // pin!(pins.gpio19);
  // pin!(pins.gpio21);

  loop {
    unsafe {
      esp_idf_sys::esp_task_wdt_reset();
    }

    if let Some(pin_id) = task::wait_notification(duration) {
      info!("Notification received: {}", pin_id);
    }
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

  let mut state_guard = CURRENT_INPUT_STATE.lock().unwrap();
  debug!("Current state: {:?}", *state_guard);

  let (event, new_state) = state_guard.transition_to(curr_state)?;
  *state_guard = new_state;

  info!("New state: {:?}", *state_guard);
  info!("New event: {:?}", event);

  // let keyboard = KEYBOARD.lock().unwrap();

  // match event {
  //   Some(BluetoothEvent::MediaControlKey(key)) => {
  //     keyboard.send_media_key(key.into());
  //   },
  //   Some(BluetoothEvent::Letter(letter)) => {
  //     KEYBOARD.lock().send_shortcut(letter);
  //   },
  //   None => {
  //     warn!("No event for button click: {:?}", curr_state);
  //   },
  // };

  Ok(())
}
