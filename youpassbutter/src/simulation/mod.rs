extern crate three;
extern crate mint;

use three::Object;
use state_machines::shared_state::{SharedState};
use state_machines::chassis::{Chassis, States};
use std::{thread, time};
use simulation_drivers;

const INITIAL_Z_OFFSET: f32 = 0.5;

pub fn run_simulation() {
  // Wcreate the chassis state machine. 
  let current_position: mint::Point3<f32> = mint::Point3 {
    x: 0.0,
    y: 0.0,
    z: INITIAL_Z_OFFSET,
  };
  let shared_state: SharedState = SharedState::new(true, current_position);
  let mut chassis: Chassis = Chassis::new(shared_state);

  let title = "Devastator Simulator";
  let mut window = three::Window::new(title);

  let light = window.factory.point_light(0xFFFFFF, 0.6);
  light.look_at([3.0, 1.0, 1.0], [4.0, 4.0, 4.0], None);
  window.scene.add(&light);

  window.scene.background = three::Background::Color(0xC6F0FF);

  let tank_geometry: three::Geometry = three::Geometry::cuboid(1.0, 1.0, 1.0);
  let tank_material: three::material::Basic = three::material::Basic {
    color: 0x555555,
    .. Default::default()
  };
  let tank_mesh: three::Mesh = window.factory.mesh(
    tank_geometry,
    tank_material
  );

  let ground_geometry: three::Geometry = three::Geometry::plane(30.0, 30.0);
    let ground_material: three::material::Basic = three::material::Basic {
    color: 0x888888,
    .. Default::default()
  };
  let ground_position: mint::Point3<f32> = mint::Point3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
  };
  let ground_mesh: three::Mesh = window.factory.mesh(
    ground_geometry,
    ground_material
  );

  window.scene.add(&ground_mesh);
  window.scene.add(&tank_mesh);

  tank_mesh.set_position(current_position);
  ground_mesh.set_position(ground_position);

  let camera = window.factory.perspective_camera(
    100.0,
    0.1 .. 40.0
  );
  let mut controls = three::controls::Orbit::builder(&camera)
    .position([2.5, 2.5, 7.0])
    .target([0.0, 0.0, 0.5])
    .build();  
  let ten_millis = time::Duration::from_millis(10);

  // The main while 
  while window.update() {
    controls.update(&window.input);
    window.render(&camera);
    thread::sleep(ten_millis);

    simulation_drivers::speed_monitor::update_position(chassis.shared_state.managed_state.clone());

    let keys: &[three::controls::Key] = window.input.keys_hit();
    if keys.len() > 0 {
      match keys[0] {
        three::controls::Key::Up => {
          if States::Backward == chassis.current_state {  
            chassis.set_state(States::Idle);
          } else {
            chassis.set_state(States::Forward);
          }
        },
        three::controls::Key::Down => {
          if States::Forward == chassis.current_state {  
            chassis.set_state(States::Idle);
          } else {
            chassis.set_state(States::Backward);
          }
        },
        three::controls::Key::Left => return,
        three::controls::Key::Right => return,
        _ => return,
      };
    }

    let mut managed_state = chassis.shared_state.managed_state.lock().unwrap();
    let position: mint::Point3<f32> = mint::Point3 {
      x: managed_state.x,
      y: managed_state.y,
      z: managed_state.z
    };
    tank_mesh.set_position(position);
  }   
}
