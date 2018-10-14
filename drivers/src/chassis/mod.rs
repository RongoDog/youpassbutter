/*
    This file should contain the interface with which the raspberry
    pi communicates with the motor driver for the chassis.
*/
extern crate rppal; 

use ressources::{HardwareInterfacePointers};

const ENABLE_A: u8 = 26;
const ENABLE_B: u8 = 22;
const IN_1: u8 = 19;
const IN_2: u8 = 13;
const IN_3: u8 = 6;
const IN_4: u8 = 5;

// Some templates
pub fn stop(pointers: HardwareInterfacePointers) -> bool {
  let mut gpio = pointers.gpio_mutex.lock().unwrap();
  gpio.write(ENABLE_A, rppal::gpio::Level::Low);
  gpio.write(ENABLE_B, rppal::gpio::Level::Low);
  gpio.write(IN_1, rppal::gpio::Level::Low);
  gpio.write(IN_2, rppal::gpio::Level::Low);
  gpio.write(IN_3, rppal::gpio::Level::Low);
  gpio.write(IN_4, rppal::gpio::Level::Low);
  return true;
}

pub fn forward(pointers: HardwareInterfacePointers) -> bool {
  let mut gpio = pointers.gpio_mutex.lock().unwrap();
  gpio.write(ENABLE_A, rppal::gpio::Level::High);
  gpio.write(ENABLE_B, rppal::gpio::Level::High);
  gpio.write(IN_2, rppal::gpio::Level::Low);
  gpio.write(IN_1, rppal::gpio::Level::High);
  gpio.write(IN_4, rppal::gpio::Level::Low);
  gpio.write(IN_3, rppal::gpio::Level::High);
  return true;
}

pub fn backward(pointers: HardwareInterfacePointers) -> bool {
  let mut gpio = pointers.gpio_mutex.lock().unwrap();
  gpio.write(ENABLE_A, rppal::gpio::Level::High);
  gpio.write(ENABLE_B, rppal::gpio::Level::High);
  gpio.write(IN_1, rppal::gpio::Level::Low);
  gpio.write(IN_2, rppal::gpio::Level::High);
  gpio.write(IN_3, rppal::gpio::Level::Low);
  gpio.write(IN_4, rppal::gpio::Level::High);
  return true;
}

// This function sets the mode for all the required GPIO pins
pub fn initialize(pointers: HardwareInterfacePointers) -> bool {
  let mut gpio = pointers.gpio_mutex.lock().unwrap();
  gpio.set_mode(ENABLE_A, rppal::gpio::Mode::Output);
  gpio.set_mode(ENABLE_B, rppal::gpio::Mode::Output);
  gpio.set_mode(IN_1, rppal::gpio::Mode::Output);
  gpio.set_mode(IN_2, rppal::gpio::Mode::Output);
  gpio.set_mode(IN_3, rppal::gpio::Mode::Output);
  gpio.set_mode(IN_4, rppal::gpio::Mode::Output);
  return true;
}