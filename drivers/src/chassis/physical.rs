/*
    This file should contain the interface with which the raspberry
    pi communicates with the motor driver for the chassis.
*/
extern crate rppal; 

const ENABLE_A: u8 = 1;
const ENABLE_B: u8 = 2;
const IN_1: u8 = 3;
const IN_2: u8 = 4;
const IN_3: u8 = 5;
const IN_4: u8 = 6;

const STEP_1: u8 = 18;
const STEP_2: u8 = 23;
const STEP_3: u8 = 24;
const STEP_4: u8 = 25;

pub fn first_step(mut gpio: rppal::gpio::Gpio) {
    gpio.write(STEP_1, rppal::gpio::Level::High);
    gpio.write(STEP_2, rppal::gpio::Level::Low);
    gpio.write(STEP_3, rppal::gpio::Level::Low);
    gpio.write(STEP_4, rppal::gpio::Level::Low);
}

pub fn second_step(mut gpio: rppal::gpio::Gpio) {
    gpio.write(STEP_1, rppal::gpio::Level::High);
    gpio.write(STEP_2, rppal::gpio::Level::High);
    gpio.write(STEP_3, rppal::gpio::Level::Low);
    gpio.write(STEP_4, rppal::gpio::Level::Low);
}

pub fn third_step(mut gpio: rppal::gpio::Gpio) {
    gpio.write(STEP_1, rppal::gpio::Level::Low);
    gpio.write(STEP_2, rppal::gpio::Level::High);
    gpio.write(STEP_3, rppal::gpio::Level::Low);
    gpio.write(STEP_4, rppal::gpio::Level::Low);
}

pub fn fourth_step(mut gpio: rppal::gpio::Gpio) {
    gpio.write(STEP_1, rppal::gpio::Level::Low);
    gpio.write(STEP_2, rppal::gpio::Level::High);
    gpio.write(STEP_3, rppal::gpio::Level::High);
    gpio.write(STEP_4, rppal::gpio::Level::Low);
}

pub fn fifth_step(mut gpio: rppal::gpio::Gpio) {
    gpio.write(STEP_1, rppal::gpio::Level::Low);
    gpio.write(STEP_2, rppal::gpio::Level::Low);
    gpio.write(STEP_3, rppal::gpio::Level::High);
    gpio.write(STEP_4, rppal::gpio::Level::Low);
}

pub fn sixth_step(mut gpio: rppal::gpio::Gpio) {
    gpio.write(STEP_1, rppal::gpio::Level::Low);
    gpio.write(STEP_2, rppal::gpio::Level::Low);
    gpio.write(STEP_3, rppal::gpio::Level::High);
    gpio.write(STEP_4, rppal::gpio::Level::High);
}

pub fn seventh_step(mut gpio: rppal::gpio::Gpio) {
    gpio.write(STEP_1, rppal::gpio::Level::Low);
    gpio.write(STEP_2, rppal::gpio::Level::Low);
    gpio.write(STEP_3, rppal::gpio::Level::Low);
    gpio.write(STEP_4, rppal::gpio::Level::High);
}

pub fn eight_step(mut gpio: rppal::gpio::Gpio) {
    gpio.write(STEP_1, rppal::gpio::Level::High);
    gpio.write(STEP_2, rppal::gpio::Level::Low);
    gpio.write(STEP_3, rppal::gpio::Level::Low);
    gpio.write(STEP_4, rppal::gpio::Level::High);
}

// Some templates
pub fn stop(mut gpio: rppal::gpio::Gpio) -> bool {
    gpio.write(ENABLE_A, rppal::gpio::Level::Low);
    gpio.write(ENABLE_B, rppal::gpio::Level::Low);
    gpio.write(IN_1, rppal::gpio::Level::Low);
    gpio.write(IN_2, rppal::gpio::Level::Low);
    gpio.write(IN_3, rppal::gpio::Level::Low);
    gpio.write(IN_4, rppal::gpio::Level::Low);
    return true;
}

pub fn forward(mut gpio: rppal::gpio::Gpio) -> bool {
    gpio.write(ENABLE_A, rppal::gpio::Level::High);
    gpio.write(ENABLE_B, rppal::gpio::Level::High);
    gpio.write(IN_2, rppal::gpio::Level::Low);
    gpio.write(IN_1, rppal::gpio::Level::High);
    gpio.write(IN_4, rppal::gpio::Level::Low);
    gpio.write(IN_3, rppal::gpio::Level::High);
    return true;
}

pub fn backward(mut gpio: rppal::gpio::Gpio) -> bool {
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
pub fn initialize(&mut gpio: rppal::gpio::Gpio) -> bool {
    gpio.set_mode(ENABLE_A, rppal::gpio::Mode::Output);
    gpio.set_mode(ENABLE_B, rppal::gpio::Mode::Output);
    gpio.set_mode(IN_1, rppal::gpio::Mode::Output);
    gpio.set_mode(IN_2, rppal::gpio::Mode::Output);
    gpio.set_mode(IN_3, rppal::gpio::Mode::Output);
    gpio.set_mode(IN_4, rppal::gpio::Mode::Output);

    gpio.set_mode(STEP_1, rppal::gpio::Mode::Output);
    gpio.set_mode(STEP_2, rppal::gpio::Mode::Output);
    gpio.set_mode(STEP_3, rppal::gpio::Mode::Output);
    gpio.set_mode(STEP_4, rppal::gpio::Mode::Output);
    //stop(gpio);
    return true;
}