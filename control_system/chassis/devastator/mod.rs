// Module that contains the states
mod states;

use chassis::states::{Idle, Forward, Backward};
use motor_drivers::physical;
use motor_drivers::simulation;
use std::time::SystemTime;

struct SharedState {
    speed_x: f32,
    speed_y: f32,
    speed_z: f32,
    x: f32,
    y: f32,
    z: f32,
    last_update: SystemTime,
    is_simulation: bool,
}

struct Devastator<T> {
    shared_state: SharedState,
};

impl Devastator<T> {
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

impl<T> From<Devastator<T>> for Devastator<Idle> {
    fn from(val: Devastator<T>) -> Devastator<Idle> {
        if (val.shared_state.is_simulation) {
            // Physical Driver
            motor_drivers::physical::stop();
        } else {
            // Simulation Driver
            motor_drivers::simulation::stop();
        }
        Devastator<Idle> {
            shared_state: val.shared_state,
            state: Idle,
        }
    }
}

impl<T> From<Devastator<T>> for Devastator<Forward> {
    fn from(val: Devastator<T>) -> Devastator<Idle> {
        if (val.shared_state.is_simulation) {
            // Physical Driver
            chassis::physical::forward();
        } else {
            // Simulation Driver
            chassis::simulation::forward();
        }
        Devastator<Idle> {
            shared_state: val.shared_state,
            state: Forward,
        }
    }
}

impl<T> From<Devastator<T>> for Devastator<Backward> {
    fn from(val: Devastator<T>) -> Devastator<Idle> {
        if (val.shared_state.is_simulation) {
            // Physical Driver
            chassis::physical::backward();
        } else {
            // Simulation Driver
            chassis::simulation::backward();
        }
        Devastator<Idle> {
            shared_state: val.shared_state,
            state: Backward,
        }
    }
}

impl Devastator<Idle> {
    pub fn new(is_simulation: bool) -> Self {
        if (val.shared_state.is_simulation) {
            // Physical Driver
            if !chassis::physical::initialize() {
                println!("Failed to initialize devastor");
            }
            chassis::physical::stop();
        } else {
            // Simulation Driver
            chassis::simulation::stop();
        }

        return Devastator<Idle> {
            is_simulation: is_simulation,
        }
    }
}