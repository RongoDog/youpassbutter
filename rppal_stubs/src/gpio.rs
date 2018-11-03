pub struct Gpio {

}

pub enum Mode {
  Input,
  Output,
}

pub enum Level {
  Low,
  High,
}

impl Gpio {
  pub fn new() -> Self {
    return Gpio {

    };
  }

  pub fn unwrap(&self) -> Self {
    return Gpio {

    };
  }

  pub fn set_mode(&self, pin: u8, m: Mode) {
    return;
  }

  pub fn write(&self, pin: u8, l: Level) {
    return;
  }
}

