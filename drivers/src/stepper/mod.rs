extern crate rppal; 

const STEP_1: u8 = 18;
const STEP_2: u8 = 23;
const STEP_3: u8 = 24;
const STEP_4: u8 = 25;



/*
Below is example code of how this module could be used. 
We may want a function that we can use to specify rotation degrees. That way we can precisely rotate 
and add semantic understanding to the robot. 

One interesting option is to launch a thread that has control over the GPIO pins (we need to consider mutual exclusion). It can interact with
the owner of the thread using interrupts. This avoids having to introduce arbitary delays in the master
control process.


EXAMPLE CODE


extern crate drivers;
extern crate rppal;

use std::{thread, time};

fn main() {
    let second = time::Duration::from_millis(1);
    let mut gpio: rppal::gpio::Gpio = rppal::gpio::Gpio::new().unwrap();
    drivers::chassis::physical::initialize(&mut gpio);

    let mut step = 1;
    loop {
        if step > 8 {
            step = 1;
        }
        match step {
            1 => drivers::stepper::first_step(&mut gpio),
            2 => drivers::stepper::second_step(&mut gpio),
            3 => drivers::stepper::third_step(&mut gpio),
            4 => drivers::stepper::fourth_step(&mut gpio),
            5 => drivers::stepper::fifth_step(&mut gpio),
            6 => drivers::stepper::sixth_step(&mut gpio),
            7 => drivers::stepper::seventh_step(&mut gpio),
            8 => drivers::stepper::eight_step(&mut gpio),
            _ => drivers::stepper::fifth_step(&mut gpio)
        }
        step += 1;
        thread::sleep(second);

    }
}



*/

pub fn zero_step(gpio: &mut rppal::gpio::Gpio) {
  gpio.write(STEP_1, rppal::gpio::Level::Low);
  gpio.write(STEP_2, rppal::gpio::Level::Low);
  gpio.write(STEP_3, rppal::gpio::Level::Low);
  gpio.write(STEP_4, rppal::gpio::Level::Low);
}

pub fn first_step(gpio: &mut rppal::gpio::Gpio) {
  gpio.write(STEP_1, rppal::gpio::Level::High);
  gpio.write(STEP_2, rppal::gpio::Level::Low);
  gpio.write(STEP_3, rppal::gpio::Level::Low);
  gpio.write(STEP_4, rppal::gpio::Level::Low);
}

pub fn second_step(gpio: &mut rppal::gpio::Gpio) {
  gpio.write(STEP_1, rppal::gpio::Level::High);
  gpio.write(STEP_2, rppal::gpio::Level::High);
  gpio.write(STEP_3, rppal::gpio::Level::Low);
  gpio.write(STEP_4, rppal::gpio::Level::Low);
}

pub fn third_step(gpio: &mut rppal::gpio::Gpio) {
  gpio.write(STEP_1, rppal::gpio::Level::Low);
  gpio.write(STEP_2, rppal::gpio::Level::High);
  gpio.write(STEP_3, rppal::gpio::Level::Low);
  gpio.write(STEP_4, rppal::gpio::Level::Low);
}

pub fn fourth_step(gpio: &mut rppal::gpio::Gpio) {
  gpio.write(STEP_1, rppal::gpio::Level::Low);
  gpio.write(STEP_2, rppal::gpio::Level::High);
  gpio.write(STEP_3, rppal::gpio::Level::High);
  gpio.write(STEP_4, rppal::gpio::Level::Low);
}

pub fn fifth_step(gpio: &mut rppal::gpio::Gpio) {
  gpio.write(STEP_1, rppal::gpio::Level::Low);
  gpio.write(STEP_2, rppal::gpio::Level::Low);
  gpio.write(STEP_3, rppal::gpio::Level::High);
  gpio.write(STEP_4, rppal::gpio::Level::Low);
}

pub fn sixth_step(gpio: &mut rppal::gpio::Gpio) {
  gpio.write(STEP_1, rppal::gpio::Level::Low);
  gpio.write(STEP_2, rppal::gpio::Level::Low);
  gpio.write(STEP_3, rppal::gpio::Level::High);
  gpio.write(STEP_4, rppal::gpio::Level::High);
}

pub fn seventh_step(gpio: &mut rppal::gpio::Gpio) {
    gpio.write(STEP_1, rppal::gpio::Level::Low);
    gpio.write(STEP_2, rppal::gpio::Level::Low);
    gpio.write(STEP_3, rppal::gpio::Level::Low);
    gpio.write(STEP_4, rppal::gpio::Level::High);
}

pub fn eight_step(gpio: &mut rppal::gpio::Gpio) {
  gpio.write(STEP_1, rppal::gpio::Level::High);
  gpio.write(STEP_2, rppal::gpio::Level::Low);
  gpio.write(STEP_3, rppal::gpio::Level::Low);
  gpio.write(STEP_4, rppal::gpio::Level::High);
}

// This function should initialize the GPIO
// NOTE THAT THIS SHOULD BE MOVED ONCES WE ADD PERIPHERALS
pub fn initialize(gpio: &mut rppal::gpio::Gpio) -> bool {
  gpio.set_mode(STEP_1, rppal::gpio::Mode::Output);
  gpio.set_mode(STEP_2, rppal::gpio::Mode::Output);
  gpio.set_mode(STEP_3, rppal::gpio::Mode::Output);
  gpio.set_mode(STEP_4, rppal::gpio::Mode::Output);
  zero_step(gpio);
  return true;
}
