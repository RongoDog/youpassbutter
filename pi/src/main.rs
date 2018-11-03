extern crate youpassbutter;
extern crate ws;

use ws::{connect, CloseCode};

fn main() {
    // Wcreate the chassis state machine. 
  let current_position: mint::Point3<f32> = mint::Point3 {
    x: 0.0,
    y: 0.0,
    z: INITIAL_Z_OFFSET,
  };
  let shared_state: SharedState = SharedState::new(true, current_position);
  let mut chassis: Chassis = Chassis::new(shared_state);

  // Connect to the url and call the closure
  if let Err(error) = connect("ws://127.0.0.1:3012", |out| {
      // Queue a message to be sent when the WebSocket is open
      if out.send("Hello WebSocket").is_err() {
          println!("Websocket couldn't queue an initial message.")
      } else {
          println!("Client sent message 'Hello WebSocket'. ")
      }
      // The handler needs to take ownership of out, so we use move
      move |msg| {
          // Handle messages received on this connection
          println!("Client got message '{}'. ", msg);
          // Close the connection
          out.close(CloseCode::Normal)
      }
  }) {
      // Inform the user of failure
      println!("Failed to create WebSocket due to: {:?}", error);
  }  
}
