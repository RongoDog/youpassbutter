// Module that contains the states
mod states;

use chassis::states::{Idle, Forward, Backward};
use std::time::SystemTime;

struct SharedState {
    speed_x: f32,
    speed_y: f32,
    speed_z: f32,
    x: f32,
    y: f32,
    z: f32,
    last_update: SystemTime,
}

struct Devastator<T> {
    shared_state: SharedState,
    state: T,
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

impl From<Devastator> for Devastator<Idle> {
    fn from(val: Devastator<T>) -> Devastator<Idle> {
        Devastator<Idle> {
            shared_state: val.shared_state,
            state: Idle,
        }
    }
}

impl From<Devastator> for Devastator<Forward> {
    fn from(val: Devastator<T>) -> Devastator<Idle> {
        Devastator<Idle> {
            shared_state: val.shared_state,
            state: Forward,
        }
    }
}

impl From<Devastator> for Devastator<Backward> {
    fn from(val: Devastator<T>) -> Devastator<Idle> {
        Devastator<Idle> {
            shared_state: val.shared_state,
            state: Backward,
        }
    }
}

impl Devastator<Idle> {
    pub fn new() -> Self {
        Devastator<Idle> {
            speed_x: 0.0,
            speed_y: 0.0,
            speed_z: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            state: Idle,
            last_update: SystemTime::now(),
        }
    }
}