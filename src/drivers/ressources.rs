extern crate rppal;

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct HardwareInterfacePointers {
  pub gpio_mutex: Arc<Mutex<rppal::gpio::Gpio>>,
}

impl HardwareInterfacePointers {
  pub fn new() -> Self {
    let mut gpio: rppal::gpio::Gpio = rppal::gpio::Gpio::new().unwrap();
    let gpio_mutex: Arc<Mutex<rppal::gpio::Gpio>> = Arc::<Mutex<rppal::gpio::Gpio>>::new(
        Mutex::<rppal::gpio::Gpio>::new(gpio)
      );
    return HardwareInterfacePointers {
      gpio_mutex: gpio_mutex
    };
  }
}