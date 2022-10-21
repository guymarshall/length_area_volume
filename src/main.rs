use std::f64::consts::PI;

mod user_input;

fn square_area(length: f64) -> f64 {
    length * length
}

fn circle_area(length: f64) -> f64 {
    (PI * length * length) / 4.0
}

fn equilateral_triangle_area(length: f64) -> f64 {
    (length * length) * (3.0_f64.cbrt() / 4.0)
}

fn cube_area(length: f64) -> f64 {
    (length * length) * 6.0
}

fn cube_volume(length: f64) -> f64 {
    length * length * length
}

fn main() {
    let length: f64 = user_input::get_user_input("Enter length of side (for a circle, length is diameter): ");

    // print area/volume of every shape

    println!("Square: area - {}", square_area(length));
    println!("Circle: area - {}", circle_area(length));
    println!("Equilateral triangle: area - {}", equilateral_triangle_area(length));

    println!("Cube: area - {}, volume: {}", cube_area(length), cube_volume(length));
    // sphere
    // tetrahedron (3D equilateral triangle)
}
