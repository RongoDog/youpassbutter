
extern crate drivers;
extern crate rppal;

use std::{thread, time};

fn main() {
    let ten_millis = time::Duration::from_millis(10);
    let mut gpio: rppal::gpio::Gpio = rppal::gpio::Gpio::new();
    drivers::chassis::initialize(mut gpio);

    let mut step = 1;
    while(1) {
        if (step > 8) {
            step = 1;
        }
        match step {
            1 => drivers::chassis::first_step(gpio),
            2 => drivers::chassis::second_step(gpio),
            3 => drivers::chassis::third_step(gpio),
            4 => drivers::chassis::fourth_step(gpio),
            5 => drivers::chassis::fifth_step(gpio),
            6 => drivers::chassis::sixth_step(gpio),
            7 => drivers::chassis::seventh_step(gpio),
            8 => drivers::chassis::eight_step(gpio)
        }
        step ++;
        thread::sleep(ten_millis*100);

    }
}