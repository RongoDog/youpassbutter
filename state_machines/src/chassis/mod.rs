extern crate drivers;

mod states;
mod user_control;

use std::time::SystemTime;
use std::sync::{Mutex};

use states::{Idle, Forward, Backward};

struct Chassis<T> {
  shared_state: Mutex<SharedState>,
};

impl Chassis<T> {
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
}

impl<T> From<Chassis<T>> for Chassis<Idle> {
  fn from(val: Chassis<T>) -> Chassis<Idle> {
    if val.shared_state.is_simulation {
      // Simulation Driver
      drivers::simulation::chassis::stop(val.shared_state);
    } else {
      // Physical Driver
      drivers::chassis::stop(val.shared_state);
    }
    return Chassis<Idle> {
      shared_state: val.shared_state,
    }
  }
}

impl<T> From<Chassis<T>> for Chassis<Forward> {
  fn from(val: Chassis<T>) -> Chassis<Forward> {
      if val.shared_state.is_simulation {
      // Simulation Driver
    drivers::simulation::chassis::forward(val.shared_state);
    } else {
      // Physical Driver
      drivers::chassis::forward(val.shared_state);
    }
    return Chassis<Forward> {
      shared_state: val.shared_state,
    }
  }
}

impl<T> From<Chassis<T>> for Chassis<Backward> {
    fn from(val: Chassis<T>) -> Chassis<Backward> {
        if val.shared_state.is_simulation {
            // Simulation Driver
            drivers::simulation::chassis::backward(val.shared_state);
        } else {
            // Physical Driver
            drivers::chassis::backward(val.shared_state);
        }
        Devastator<Backward> {
            shared_state: val.shared_state,
        }
    }
}

impl Chassis<Idle> {
    pub fn new(shared_state: Mutex<SharedState>) -> Self {
        if !val.shared_state.is_simulation {
            // Physical Driver
            if !drivers::chassis::initialize(shared_state.gpio) {
                println!("Failed to initialize devastor");
            }
            drivers::chassis::stop(shared_state.gpio);
        } else {
            drivers::chassis::stop(shared_state.gpio);
        }
        return Chassis<Idle> {
            shared_state: shared_state,
        };
    }
}