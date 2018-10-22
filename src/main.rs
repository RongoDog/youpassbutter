extern crate youpassbutter;
extern crate mint;

use youpassbutter::state_machines::shared_state::{SharedState};
use youpassbutter::state_machines::chassis::{Chassis, States};

fn main() {
  // We create the chassis state machine. 
  let initial_position: mint::Point3<f32> = mint::Point3 {
    x: 0.0, 
    y: 0.0,
    z: 0.0, // To be determined based on what we deem the center.
  };
  let shared_state: SharedState = SharedState::new(true, current_position);
  let mut chassis: Chassis = Chassis::new(shared_state);
  

}
