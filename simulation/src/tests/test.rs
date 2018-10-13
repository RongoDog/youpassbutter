
const RADIUS: f32 = 1.0;

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
