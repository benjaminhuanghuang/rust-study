struct Coordinate<T> {
    x: T,
    y: T,
    z: T
}

pub fn do_it() {

    println!("\nIn demo_generic_structs::do_it()");

    let c1 = Coordinate::<i32> {x: 1, y: 2, z: 3};
    println!("{} {} {}", c1.x, c1.y, c1.z);

    let c2 = Coordinate {x: 1.1, y: 2.2, z: 3.3};
    println!("{} {} {}", c2.x, c2.y, c2.z);
}