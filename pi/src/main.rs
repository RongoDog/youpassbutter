
extern crate drivers;
extern crate rppal;

use std::{thread, time};

fn main() {
    let ten_millis = time::Duration::from_millis(10);
    let mut gpio: rppal::gpio::Gpio = rppal::gpio::Gpio::new().unwrap();
    drivers::chassis::physical::initialize(&mut gpio);

    let mut step = 1;
    loop {
        if step > 8 {
            step = 1;
        }
        match step {
            1 => drivers::chassis::physical::first_step(&mut gpio),
            2 => drivers::chassis::physical::second_step(&mut gpio),
            3 => drivers::chassis::physical::third_step(&mut gpio),
            4 => drivers::chassis::physical::fourth_step(&mut gpio),
            5 => drivers::chassis::physical::fifth_step(&mut gpio),
            6 => drivers::chassis::physical::sixth_step(&mut gpio),
            7 => drivers::chassis::physical::seventh_step(&mut gpio),
            8 => drivers::chassis::physical::eight_step(&mut gpio),
            _ => drivers::chassis::physical::fifth_step(&mut gpio)
        }
        step += 1;
        thread::sleep(ten_millis*10);

    }
}