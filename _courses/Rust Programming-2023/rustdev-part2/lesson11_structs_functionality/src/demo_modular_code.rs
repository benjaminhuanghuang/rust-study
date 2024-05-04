use crate::mytypes::point::Point;

pub fn do_it() {

    println!("\nIn demo_modular_code::do_it()");

    let mut p1 = Point {x: 10, y: 20};

    p1.move_by(100, 200);
    println!("{}", p1.to_string());

    p1.reset();
    println!("{}", p1.to_string());
}