/*
    This file should contain the interface with which the raspberry
    pi communicates with the motor driver for the chassis.
*/
extern crate rppal; 

const ENABLE_A: u32 = 1;
const ENABLE_B: u32 = 2;
const IN_1: u32 = 3;
const IN_2: u32 = 4;
const IN_3: u32 = 5;
const IN_4: u32 = 6;

// Some templates
fn stop(gpio: rppal::gpio::Gpio) -> bool {
    gpio.write(ENABLE_A, rppal::gpio::Level::Low);
    gpio.write(ENABLE_B, rppal::gpio::Level::Low);
    gpio.write(IN_1, rppal::gpio::Level::Low);
    gpio.write(IN_2, rppal::gpio::Level::Low);
    gpio.write(IN_3, rppal::gpio::Level::Low);
    gpio.write(IN_4, rppal::gpio::Level::Low);
    return true;
}

fn forward(gpio: rppal::gpio::Gpio) -> bool {
    gpio.write(ENABLE_A, rppal::gpio::Level::High);
    gpio.write(ENABLE_B, rppal::gpio::Level::High);
    gpio.write(IN_2, rppal::gpio::Level::Low);
    gpio.write(IN_1, rppal::gpio::Level::High);
    gpio.write(IN_4, rppal::gpio::Level::Low);
    gpio.write(IN_3, rppal::gpio::Level::High);
    return true;
}

fn backward(gpio: rppal::gpio::Gpio) -> bool {
    gpio.write(ENABLE_A, rppal::gpio::Level::High);
    gpio.write(ENABLE_B, rppal::gpio::Level::High);
    gpio.write(IN_1, rppal::gpio::Level::Low);
    gpio.write(IN_2, rppal::gpio::Level::High);
    gpio.write(IN_3, rppal::gpio::Level::Low);
    gpio.write(IN_4, rppal::gpio::Level::High);
    return true;
}

// This function should initialize the GPIO
// NOTE THAT THIS SHOULD BE MOVED ONCES WE ADD PERIPHERALS
fn initialize(gpio: rppal::gpio::Gpio) -> bool {
    gpio.set_mode(ENABLE_A, rppal::gpio::Mode::Output);
    gpio.set_mode(ENABLE_B, rppal::gpio::Mode::Output);
    gpio.set_mode(IN_1, rppal::gpio::Mode::Output);
    gpio.set_mode(IN_2, rppal::gpio::Mode::Output);
    gpio.set_mode(IN_3, rppal::gpio::Mode::Output);
    gpio.set_mode(IN_4, rppal::gpio::Mode::Output);
    stop(gpio);
    return true;
}