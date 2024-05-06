use crate::mytypes::point3d::Point3D;

pub fn do_it() {

    println!("\nIn demo_associated_functions::do_it()");

    let mut p1 = Point3D::new(10, 20, 30);

    p1.move_by(100, 200, 300);
    println!("{}", p1.to_string());

    p1.reset();
    println!("{}", p1.to_string());
}