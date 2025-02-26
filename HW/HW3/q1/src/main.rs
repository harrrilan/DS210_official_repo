enum Shape {
    Pyramid { length: f64, width: f64, height: f64 },
    Cuboid { length: f64, width: f64, height: f64 },
    Sphere { radius: f64 },
}

impl Shape {

    
    fn volume(&self) -> f64 {
        match self {
            Shape::Pyramid { length, width, height } => length * width * height / 3.0,
            Shape::Cuboid { length, width, height } => length * width * height,
            Shape::Sphere { radius } => 4.0 / 3.0 * std::f64::consts::PI * radius.powf(3.0),
        }
    }

    fn surface_area(&self) -> f64 {
        match self {
            Shape::Pyramid { length, width, height } => {
                length * width +
                length * f64::sqrt((width / 2.0) * (width / 2.0) + height * height) +
                width * f64::sqrt((length / 2.0) * (length / 2.0) + height * height)
            }
            Shape::Cuboid { length, width, height } => {
                2.0 * (length * width + width * height + length * height)
            }
            Shape::Sphere { radius } => {
                4.0 * std::f64::consts::PI * radius * radius
            }
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            Shape::Pyramid { length, width, height } => *length > 0.0 && *width > 0.0 && *height > 0.0,
            Shape::Cuboid { length, width, height } => *length > 0.0 && *width > 0.0 && *height > 0.0,
            Shape::Sphere { radius } => *radius > 0.0,
        }
    }

    fn double_height_or_radius(&self) -> Shape {
        match self {
            Shape::Pyramid { length, width, height } => Shape::Pyramid {
                length: *length,
                width: *width,
                height: 2.0 * height,
            },
            Shape::Cuboid { length, width, height } => Shape::Cuboid {
                length: *length,
                width: *width,
                height: 2.0 * height,
            },
            Shape::Sphere { radius } => Shape::Sphere {
                radius: 2.0 * radius,
            },
        }
    }
}

fn create_shape(shape_type: &str, l_or_r: f64, w: f64, h: f64) -> Shape {
    match shape_type {
        "pyramid" => Shape::Pyramid { length: l_or_r, width: w, height: h },
        "cuboid" => Shape::Cuboid { length: l_or_r, width: w, height: h },
        "sphere" => Shape::Sphere { radius: l_or_r },
        _ => panic!("Unknown shape type! Use 'pyramid', 'cuboid', or 'sphere'"),
    }
}

fn main() {
    let pyramid = create_shape("pyramid", 2.0, 3.0, 4.0);
    println!("Pyramid:");
    println!("  Volume: {}", pyramid.volume());
    println!("  Surface Area: {}", pyramid.surface_area());
    println!("  Is Valid: {}", pyramid.is_valid());
    let doubled_pyramid = pyramid.double_height_or_radius();
    println!("  Volume after doubling height: {}", doubled_pyramid.volume());
    
    let cuboid = create_shape("cuboid", 2.0, 3.0, 4.0);
    println!("\nCuboid:");
    println!("  Volume: {}", cuboid.volume());
    println!("  Surface Area: {}", cuboid.surface_area());
    println!("  Is Valid: {}", cuboid.is_valid());
    let doubled_cuboid = cuboid.double_height_or_radius();
    println!("  Volume after doubling height: {}", doubled_cuboid.volume());
    
    let sphere = create_shape("sphere", 2.0, 0.0, 0.0); // w and h ignored for sphere
    println!("\nSphere:");
    println!("  Volume: {}", sphere.volume());
    println!("  Surface Area: {}", sphere.surface_area());
    println!("  Is Valid: {}", sphere.is_valid());
    let doubled_sphere = sphere.double_height_or_radius();
    println!("  Volume after doubling radius: {}", doubled_sphere.volume());
}