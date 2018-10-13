use chassis::{Chassis};
use chassis::states::{States};

enum ChassisWrapper {
    Idle(Chassis<States::Idle>),
    Forward(Chassis<States::Forward>),
    Background(Chassis<States::Background>),
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