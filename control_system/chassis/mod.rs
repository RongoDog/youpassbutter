mod devastator;

use std::{thread, time};

const SLEEP_INTERVAL: f32 = 10.0;


use std::{thread, time};
let ten_millis = time::Duration::from_millis(SLEEP_INTERVAL);

fn user_control_loop() {

}

fn control_system_initiation(devastator: &mut Devastator, manual_mode: bool) -> bool {

  // We check if we're in manual mode. 
  if (manual_mode) { 

  }

  while(true) {
    // We introduce a delay since there is no actual I/O for now

    thread::sleep(ten_millis);
  }
}