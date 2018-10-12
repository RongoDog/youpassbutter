/*
    This file should contain the interface with which the raspberry
    pi communicates with the motor driver for the chassis.
*/
extern crate rppal; 

use rppal::gpio;
use rppal::gpio::{Gpio, Output, Level};
use rppal::Level::{High, Low};

const ENABLE_A: u32 = 1;
const ENABLE_B: u32 = 2;
const IN_1: u32 = 3;
const IN_2: u32 = 4;
const IN_3: u32 = 5;
const IN_4: u32 = 6;

// Some templates
fn stop(gpio: Gpio) -> bool {
    gpio.write(ENABLE_A, Low);
    gpio.write(ENABLE_B, Low);
    gpio.write(IN_1, Low);
    gpio.write(IN_2, Low);
    gpio.write(IN_3, Low);
    gpio.write(IN_4, Low);
    return true;
}

fn forward(gpio: Gpio) -> bool {
    gpio.write(ENABLE_A, High);
    gpio.write(ENABLE_B, High);
    gpio.write(IN_2, Low);
    gpio.write(IN_1, High);
    gpio.write(IN_4, Low);
    gpio.write(IN_3, High);
    return true;
}

fn backward(gpio: Gpio) -> bool {
    gpio.write(ENABLE_A, High);
    gpio.write(ENABLE_B, High);
    gpio.write(IN_1, Low);
    gpio.write(IN_2, High);
    gpio.write(IN_3, Low);
    gpio.write(IN_4, High);
    return true;
}

// This function should initialize the GPIO
// NOTE THAT THIS SHOULD BE MOVED ONCES WE ADD PERIPHERALS
fn initialize(gpio: gpio::Gpio) -> bool {
    gpio.set_mode(ENABLE_A, gpio::Output);
    gpio.set_mode(ENABLE_B, gpio::Output);
    gpio.set_mode(IN_1, gpio::Output);
    gpio.set_mode(IN_2, gpio::Output);
    gpio.set_mode(IN_3, gpio::Output);
    gpio.set_mode(IN_4, gpio::Output);
    stop(gpio);
    return true;
}