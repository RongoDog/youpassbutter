/*
use chassis::{Chassis};
use chassis::states::{Idle, Forward, Backward};

enum ChassisWrapper {
    Idle(Chassis<States::Idle>),
    Forward(Chassis<States::Forward>),
    Backward(Chassis<States::Backward>),
}

struct Factory {
    chassis: ChassisWrapper,
}

impl Factory {
    fn new() -> Self {
        Factory {
            chassis: ChassisWrapper::Idle(Chassis::new(0)),
        }
    }
}
*/