
use chassis::{Chassis};
use chassis::states::{Idle, Forward, Backward};

pub struct Robot {
  pub shared_state: SharedState,
  pub chassis: Chassis,
}

impl Robot {
  fn new() -> Self {
    // We create the chassis state machine. 
    let initial_position: mint::Point3<f32> = mint::Point3 {
        x: 0.0, 
        y: 0.0,
        z: 0.0, // To be determined based on what we deem the center.
    };
    let shared_state: SharedState = SharedState::new(true, current_position);
    let mut chassis: Chassis = Chassis::new(shared_state);
    return Robot {
      shared_state: shared_state,
      chassis: chassis,
    };
  }
}
