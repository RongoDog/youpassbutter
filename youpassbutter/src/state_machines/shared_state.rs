extern crate drivers;

use std::sync::{Arc, Mutex};
use std::time::{SystemTime};


pub struct SharedState {
  pub is_simulation: bool,
  pub managed_state: Arc<Mutex<ManagedState>>,
  pub hardware_pointers: drivers::ressources::HardwareInterfacePointers,
}

pub struct ManagedState {
  pub speed_x: f32,
  pub speed_y: f32,
  pub speed_z: f32,
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub last_update: SystemTime,  
}

impl SharedState {
  pub fn new(is_simulation: bool) -> Self {
    let mut hardware_pointers: drivers::ressources::HardwareInterfacePointers; 
    if !is_simulation {
      hardware_pointers = drivers::ressources::HardwareInterfacePointers::new();
    }
    let managed_state_mutex: Arc<Mutex<ManagedState>> = Arc::<Mutex<ManagedState>>::new(Mutex::<ManagedState>::new(ManagedState {
      speed_x: 0.0,
      speed_y: 0.0,
      speed_z: 0.0,
      x: 0.0,
      y: 0.0,
      z: 0.0,
      last_update: SystemTime::now(),
    }));
    return SharedState {
      is_simulation: is_simulation,
      managed_state: managed_state_mutex,
      hardware_pointers: hardware_pointers,
    };
  }
}
