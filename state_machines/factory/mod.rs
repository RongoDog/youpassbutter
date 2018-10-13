use chassis::{Devastator, States};

enum DevastatorWrapper {
    Idle(Devastator<States::Idle>),
    Forward(Devastator<States::Forward>),
    Background(Devastator<States::Background>),
}

struct Factory {
    devastator: DevastatorWrapper,
}

impl Factory {
    fn new() -> Self {
        Factory {
            devastator: DevastatorWrapper::Idle(Devastator::new(0)),
        }
    }
}