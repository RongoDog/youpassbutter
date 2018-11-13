use std::sync::{Arc, Mutex};
use state_machines::shared_state::{ManagedState};

pub fn stop(managed_state_mutex: Arc<Mutex<ManagedState>>) -> bool {
  let mut managed_state = managed_state_mutex.lock().unwrap();
  managed_state.speed_x = 0.0;
  managed_state.speed_y = 0.0;
  managed_state.speed_z = 0.0;
  println!("Going idle!");
  return true;
}

pub fn forward(managed_state_mutex: Arc<Mutex<ManagedState>>) -> bool {
  let mut managed_state = managed_state_mutex.lock().unwrap();
  managed_state.speed_x = 0.0;
  managed_state.speed_y = 1.0;
  managed_state.speed_z = 0.0;
  println!("Going forward!");
  return true;
}

pub fn backward(managed_state_mutex: Arc<Mutex<ManagedState>>) -> bool {
  let mut managed_state = managed_state_mutex.lock().unwrap();
  managed_state.speed_x = 0.0;
  managed_state.speed_y = -1.0;
  managed_state.speed_z = 0.0;
  println!("Going backward!");
  return true;
}