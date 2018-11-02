extern crate drivers;

pub mod user_control;

use state_machines::shared_state::{SharedState};
use simulation_drivers;

#[derive(PartialEq)]
pub enum States {
  Forward,
  Backward,
  Idle,
}

pub struct Chassis {
  pub shared_state: SharedState,
  pub current_state: States,
}

impl Chassis {
  pub fn new(shared_state: SharedState) -> Self {
    if !shared_state.is_simulation {
      // Physical Driver
      if !drivers::chassis::initialize(shared_state.hardware_pointers.clone()) {
        println!("Failed to initialize devastor");
      }
      drivers::chassis::stop(shared_state.hardware_pointers.clone());
    } else {
      simulation_drivers::chassis::stop(shared_state.managed_state.clone());
    }
    return Chassis {
      shared_state: shared_state,
      current_state: States::Idle
    };
  }

  pub fn set_state(&mut self, state: States) {
    self.current_state = state;
    match self.current_state {
      States::Forward => self.forward(),
      States::Backward => self.backward(),
      States::Idle => self.idle(),
    };
    return;
  }

  fn forward(&self) {
    if self.shared_state.is_simulation {
      // Simulation Driver
      simulation_drivers::chassis::forward(self.shared_state.managed_state.clone());
    } else {
      // Physical Driver
      drivers::chassis::forward(self.shared_state.hardware_pointers.clone());
    }
  }

  fn backward(&self) {
    if self.shared_state.is_simulation {
      // Simulation Driver
      simulation_drivers::chassis::backward(self.shared_state.managed_state.clone());
    } else {
      // Physical Driver
      drivers::chassis::backward(self.shared_state.hardware_pointers.clone());
    }
  }

  fn idle(&self) {
    if self.shared_state.is_simulation {
      // Simulation Driver
      simulation_drivers::chassis::stop(self.shared_state.managed_state.clone());
    } else {
      // Physical Driver
      drivers::chassis::stop(self.shared_state.hardware_pointers.clone());
    }
  }
}