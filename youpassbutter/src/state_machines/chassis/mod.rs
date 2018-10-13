extern crate drivers;

pub mod states;
pub mod user_control;

use state_machines::shared_state::{SharedState};
use simulation_drivers;

enum States {
  Forward,
  Backward,
  Idle,
}

pub struct Chassis {
  shared_state: SharedState,
  state: States,
}

impl Chassis {
  pub fn new(shared_state: SharedState) -> Self {
    if !shared_state.is_simulation {
      // Physical Driver
      if !drivers::chassis::initialize(shared_state.hardware_pointers) {
        println!("Failed to initialize devastor");
      }
      drivers::chassis::stop(shared_state.hardware_pointers);
    } else {
      simulation_drivers::chassis::stop(shared_state.managed_state);
    }
    return Chassis {
      shared_state: shared_state,
      current_state: States::Idle
    };
  }

  pub fn set_state(&self, state: States) {
    self.state = state;
    match state {
      States::Forward => self.forward(),
      States::Backward => self.backward(),
      States::Idle => self.idle(),
      _ => return,
    };
    return;
  }

  fn forward(&self) {
    if self.shared_state.is_simulation {
      // Simulation Driver
      simulation_drivers::chassis::forward(self.shared_state.managed_state);
    } else {
      // Physical Driver
      drivers::chassis::forward(self.shared_state.hardware_pointers);
    }
  }

  fn backward(&self) {
    if self.shared_state.is_simulation {
      // Simulation Driver
      simulation_drivers::chassis::backward(self.shared_state.managed_state);
    } else {
      // Physical Driver
      drivers::chassis::backward(self.shared_state.hardware_pointers);
    }
  }

  fn idle(&self) {
    if self.shared_state.is_simulation {
      // Simulation Driver
      simulation_drivers::chassis::stop(self.shared_state.managed_state);
    } else {
      // Physical Driver
      drivers::chassis::stop(self.shared_state.hardware_pointers);
    }
  }
    /*
  pub fn move(&mut self) -> bool {
    match self.shared_state.last_update.elapsed() {
      Ok(elapsed) {
        let time_as_millis = elapsed.as_millis();
        self.shared_state.x += self.shared_state.speed_x*time_as_millis;
        self.shared_state.y += self.shared_state.speed_y*time_as_millis;
        self.shared_state.z += self.shared_state.speed_z*time_as_millis;
        self.shared_state.last_update = SystemTime::now();
      }
      Err(e) => {
        // an error occurred!
        println!("Failed to update position: {:?}", e);
      }
    }
  }
  */
}