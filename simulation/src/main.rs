extern crate three;
extern crate mint;

use std::{thread, time};
use three::Object;

const PARTS: u32 = 3;
const INITIAL_Z_OFFSET: f32 = 0.5;

fn main() {
    let title = "Devastator Simulator";
    
    let mut window = three::Window::new(title);

    let light = window.factory.point_light(0xFFFFFF, 0.6);
    light.look_at([3.0, 1.0, 1.0], [4.0, 4.0, 4.0], None);
    window.scene.add(&light);

    window.scene.background = three::Background::Color(0xC6F0FF);

    let mut current_position: mint::Point3<f32> = mint::Point3 {
        x: 0.0,
        y: 0.0,
        z: INITIAL_Z_OFFSET,
    };

    let tank_geometry: three::Geometry = three::Geometry::cuboid(1.0, 1.0, 1.0);
    let tank_material: three::material::Basic = three::material::Basic {
        color: 0x555555,
        .. Default::default()
    };
    let mut tank_mesh: three::Mesh = window.factory.mesh(
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
    let mut ground_mesh: three::Mesh = window.factory.mesh(
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
    }   

}

