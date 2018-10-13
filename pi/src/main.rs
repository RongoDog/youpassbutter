
extern crate drivers;
extern crate rppal;

use std::{thread, time};

fn main() {
    let ten_millis = time::Duration::from_millis(10);
    let mut gpio: rppal::gpio::Gpio = rppal::gpio::Gpio::new().unwrap();
    drivers::chassis::initialize(gpio);

    let mut step = 1;
    while true {
        if step > 8 {
            step = 1;
        }
        match step {
            1 => drivers::chassis::physical::first_step(gpio),
            2 => drivers::chassis::physical::second_step(gpio),
            3 => drivers::chassis::physical::third_step(gpio),
            4 => drivers::chassis::physical::fourth_step(gpio),
            5 => drivers::chassis::physical::fifth_step(gpio),
            6 => drivers::chassis::physical::sixth_step(gpio),
            7 => drivers::chassis::physical::seventh_step(gpio),
            8 => drivers::chassis::physical::eight_step(gpio)
        }
        step ++;
        thread::sleep(ten_millis*100);

    }
}