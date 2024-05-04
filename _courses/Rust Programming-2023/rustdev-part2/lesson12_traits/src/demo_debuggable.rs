use crate::mystructs::debuggable::Coord;

pub fn do_it() {

    println!("\nIn demo_debuggable::do_it()");
    
    let c = Coord::new(51.62, 3.94);
    println!("{:?}", c);
}