extern crate rppal;

use std::sync::{Arc, Mutex};
use rppal::gpio::{Gpio};

struct SharedState {
  is_simulation: bool,
  managed_state: Mutex<ManagedState>,
  gpio: Mutex<&mut Gpio>,
};

struct ManagedState {
  speed_x: f32,
  speed_y: f32,
  speed_z: f32,
  x: f32,
  y: f32,
  z: f32,
  last_update: SystemTime,  
}

impl SharedState {
  pub fn new(is_simulation: bool) -> Self {
    let mut gpio_mutex:: Gpio; 
    if !is_simulation {
      let mut gpio: Gpio = Gpio::new().unwrap();
      let gpio_mutex: Mutex<&mut Gpio> = Arc::new(Mutex<&mut Gpio>::new(&mut gpio));
    }
    let managed_state_mutex: Mutex<ManagedState> = Arc::new(Mutex<ManagedState>::new(ManagedState {
      speed_x: 0.0,
      speed_y: 0.0,
      speed_z: 0.0,
      x: 0.0,
      y: 0.0,
      z: 0.0,
      last_update: SystemTime::now(),
    });
    return SharedState {
      is_simulation: is_simulation,
      managed_state: managed_state_mutex,
      gpio: gpio_mutex,
    };
  }
}
