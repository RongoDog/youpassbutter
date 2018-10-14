use state_machines::shared_state::{ManagedState};
use std::time::{SystemTime};
use std::sync::{Arc, Mutex};

// This should be multithreaded
pub fn update_position(managed_state: Arc<Mutex<ManagedState>>) {
  let mut state = managed_state.lock().unwrap();
  match state.last_update.elapsed() {
    Ok(elapsed) => {
      let time_as_millis = elapsed.subsec_millis() as f32;
      state.x += state.speed_x*(time_as_millis as f32)/1000.0;
      state.y += state.speed_y*(time_as_millis as f32)/1000.0;
      state.z += state.speed_z*(time_as_millis as f32)/1000.0;
      state.last_update = SystemTime::now();
    }
    Err(e) => {
      // an error occurred!
      println!("Failed to update position: {:?}", e);
    }
  }
}