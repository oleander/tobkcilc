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

  let mut pin0 = pins.gpio0;

  let mut input = PinDriver::input(&mut pin0).unwrap();

  unsafe { input.subscribe(callback); }

  input.set_pull(Pull::Up).unwrap();

  // Do i need to enable anything else?
  // let mut pin = pin0.
  // pin.set_interrupt(gpio_int_type_t_GPIO_INTR_POSEDGE)?;

  // let pull = Pull::Up;
  // unsafe { gpio_set_pull_mode(pin0.pin(), pull.into()) };

  // unsafe {
  //   gpio_isr_handler_add(
  //     pin0.pin(),
  //     Some(cb),
  //     0 as *mut _,
  //   );
  // }

  // // Configure the GPIO for interrupt on a rising edge
  // unsafe {
  //   let mut io_conf = gpio_config_t {
  //     pin_bit_mask: 1 << 0, // GPIO0
  //     mode: gpio_mode_t_GPIO_MODE_INPUT,
  //     pull_up_en: gpio_pullup_t_GPIO_PULLUP_DISABLE,
  //     pull_down_en: gpio_pulldown_t_GPIO_PULLDOWN_ENABLE,
  //     intr_type: gpio_int_type_t_GPIO_INTR_POSEDGE,
  //     ..Default::default()
  //   };
  //   gpio_config(&mut io_conf);
  // }

  // Enable interrupts globally
  unsafe {
    gpio_install_isr_service(0);
  }

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
