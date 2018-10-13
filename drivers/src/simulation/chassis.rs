extern crate state_machines;

use std::sync::{Mutex};
use state_machines::shared_state::{ManagedState};

pub fn stop(managed_state_mutex: Mutex<ManagedState>) -> bool {
  let mut managed_state: ManagedState = managed_state_mutex.lock().unwrap();
  managed_state.speed_x = 0;
  managed_state.speed_y = 0;
  managed_state.speed_z = 0;
  return true;
}

pub fn forward(managed_state_mutex: Mutex<ManagedState>)  -> bool {
  let mut managed_state: ManagedState = managed_state_mutex.lock().unwrap();
  managed_state.speed_x = 0;
  managed_state.speed_y = 1;
  managed_state.speed_z = 0;
  return true;
}

pub fn backward(managed_state_mutex: Mutex<ManagedState>)  -> bool {
  let mut managed_state: ManagedState = managed_state_mutex.lock().unwrap();
  managed_state.speed_x = 0;
  managed_state.speed_y = -1;
  managed_state.speed_z = 0;
  return true;
}