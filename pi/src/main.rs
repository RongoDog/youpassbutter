
extern crate youpassbutter;
extern crate mint;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use ws::{connect, CloseCode};
use youpassbutter::state_machines::shared_state::{SharedState};
use youpassbutter::state_machines::chassis::{Chassis};

use std::{thread, time};

fn main() {
    // Wcreate the chassis state machine. 
  let current_position: mint::Point3<f32> = mint::Point3 {
    x: 0.0,
    y: 0.0,
    z: 0.5,
  };
  let shared_state: SharedState = SharedState::new(true, current_position);
  let mut chassis: Chassis = Chassis::new(shared_state);

  loop {
    thread::sleep(time::Duration::from_millis(100));

    // Core is the Tokio event loop used for making a non-blocking request
    let mut core = Core::new().unwrap();

    let client = Client::new(&core.handle());

    let url : Uri = "http://127.0.0.1:3000".parse().unwrap();

    let request = client.get(url)
        .map(|res| {
            assert_eq!(res.status(), hyper::Ok);
            println!(res);
        });

    // request is a Future, futures are lazy, so must explicitly run
    core.run(request).unwrap();

  }
}
