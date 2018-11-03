extern crate youpassbutter;

fn main() {
    // Wcreate the chassis state machine. 
  let current_position: mint::Point3<f32> = mint::Point3 {
    x: 0.0,
    y: 0.0,
    z: INITIAL_Z_OFFSET,
  };
  let shared_state: SharedState = SharedState::new(true, current_position);
  let mut chassis: Chassis = Chassis::new(shared_state);
  
}
