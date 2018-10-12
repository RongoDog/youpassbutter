extern crate three;
extern crate mint;

use std::{thread, time};
use three::Object;

const RADIUS: f32 = 1.0;
const PARTS: u32 = 3;
const INITIAL_Y_OFFSET: f32 = 0.5;

fn get_second_point(x: f32) -> f32 {
    return (RADIUS.powf(2.0) - x.powf(2.0)).sqrt();
}


// We try to make a cirlce
fn get_circle(origin: mint::Point3<f32>) -> three::Geometry {
    let mut vertices: Vec<mint::Point3<f32>> = Vec::new();
    let mut faces: Vec<[u32; 3]> = Vec::new();
    // First we add the center vertex
    vertices.push(origin);

    // First, we can create our possible x values
    for x in 0..PARTS {
        let x_val: f32 = (-1.0 * RADIUS) + (x as f32)*((2.0 * RADIUS)/(PARTS as f32 - 1.0));
        let y_val: f32 = get_second_point(x_val.abs());

        vertices.push(mint::Point3 {
            x: x_val + origin.x,
            y: y_val + origin.y,
            z: origin.z,
        });

        if x > 0 {
            faces.push([x+1, x, 0]);
            faces.push([x, x+1, 0]);
        }
    }

    for x in 0..PARTS {
        let x_val: f32 = (-1.0 * RADIUS) + (x as f32)*((2.0 * RADIUS)/(PARTS as f32 - 1.0));
        let y_val: f32 = -1.0 * get_second_point(x_val.abs());

        vertices.push(mint::Point3 {
            x: x_val + origin.x,
            y: y_val + origin.y,
            z: origin.z,
        });

        if x > 0 {
            faces.push([x+1+PARTS, x+PARTS, 0]);
            faces.push([x+PARTS, x+1+PARTS, 0]);
        }
    }

    let geometry: three::Geometry = three::Geometry {
        faces,
        base: three::Shape {
            vertices,
            .. three::Shape::default()
        },
        .. three::Geometry::default()
    };

    return geometry;
}

fn main() {
    let title = "Devastator Simulator";
    let mut window = three::Window::new(title);

    let mut current_position: mint::Point3<f32> = mint::Point3 {
        x: 0.0,
        y: INITIAL_Y_OFFSET,
        z: 20.0,
    };

    let tank_geometry: three::Geometry = three::Geometry::cuboid(1.0, 1.0, 1.0);
    let tank_material: three::material::Basic = three::material::Basic {
        color: 0x0F0F0F,
        .. Default::default()
    };
    let mut tank_mesh: three::Mesh = window.factory.mesh(
        tank_geometry,
        tank_material
    );

    let ground_geometry: three::Geometry = three::Geometry::plane(20.0, 20.0);
        let ground_material: three::material::Basic = three::material::Basic {
        color: 0x0F0F0F,
        .. Default::default()
    };
    let mut ground_mesh: three::Mesh = window.factory.mesh(
        ground_geometry,
        ground_material
    );

    window.scene.add(&ground_mesh);
    window.scene.add(&tank_mesh);

    let camera = window.factory.perspective_camera(
        60.0,
        0.1 ..
    );

    let hundred_millis = time::Duration::from_millis(100);
    while window.update() {
        tank_mesh.set_position(current_position);
        ground_mesh.set_position(current_position);
        window.render(&camera);
        thread::sleep(hundred_millis);
    }   

}

fn test() {
    let title = "Devastator initial simulation";
    let mut window = three::Window::new(title);
    let mut current_position: mint::Point3<f32> = mint::Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
 
    let mut geometry: three::Geometry = get_circle(current_position);   
    let material = three::material::Basic {
        color: 0xFFFF00,
        .. Default::default()
    };

    let mut mesh: three::DynamicMesh = window.factory.mesh_dynamic(
        geometry,
        material
    );
    window.scene.add(&mesh);    
    let center = [0.0, 0.0];
    let yextent = 20.0;
    let zrange = -20.0 .. 20.0;
    let camera = window.factory.orthographic_camera(center, yextent, zrange);   
    println!("{}", mesh.vertex_count());

    let hundred_millis = time::Duration::from_millis(10);
    let mut direction_x = 1.0;
    let mut direction_y = 1.0;
    while window.update() {
        if current_position.x > 20.0 {
            direction_x = -1.0;
        }
        if current_position.x < 0.0 {
            direction_x = 1.0;
        }           
        if current_position.y > 20.0 {
            direction_y = -1.0;
        }
        if current_position.y < 0.0 {
            direction_y = 1.0;
        }   
        current_position.x += direction_x;
        current_position.y += direction_y;
        mesh.set_position(current_position);
        window.render(&camera);
        thread::sleep(hundred_millis);
    }
}

