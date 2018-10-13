extern crate rppal;

use std::sync::{Mutex};
use rppal::gpio::{Gpio};

struct HardwareInterfacePointers {
  gpio_mutex: Mutex<&mut Gpio>,
}
