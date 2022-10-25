use std::f64::consts::PI;

mod user_input;

fn square_area(length: f64) -> f64 {
    length * length
}

fn circle_area(length: f64) -> f64 {
    (PI * length * length) / 4.0
}

fn equilateral_triangle_area(length: f64) -> f64 {
    (length * length) * (3.0_f64.sqrt() / 4.0)
}

fn cube_area(length: f64) -> f64 {
    (length * length) * 6.0
}

fn cube_volume(length: f64) -> f64 {
    length * length * length
}

fn sphere_area(length: f64) -> f64 {
    4.0 * PI * (length * length)
}

fn sphere_volume(length: f64) -> f64 {
    (4.0 / 3.0) * PI * (length * length * length)
}

fn tetrahedron_area(length: f64) -> f64 {
    length * length * (3.0_f64.sqrt())
}

fn tetrahedron_volume(length: f64) -> f64 {
    (length * length * length) / (6.0 * 2.0_f64.sqrt())
}

fn main() {
    let length: f64 = user_input::get_user_input("Enter length of side (for a circle, length is diameter): ");

    println!("Square: area - {}", square_area(length));
    println!("Circle: area - {}", circle_area(length));
    println!("Equilateral triangle: area - {}", equilateral_triangle_area(length));

    println!("Cube: area - {}, volume: {}", cube_area(length), cube_volume(length));
    println!("Sphere: area - {}, volume: {}", sphere_area(length), sphere_volume(length));
    println!("Tetrahedron: area - {}, volume: {}", tetrahedron_area(length), tetrahedron_volume(length));
}
