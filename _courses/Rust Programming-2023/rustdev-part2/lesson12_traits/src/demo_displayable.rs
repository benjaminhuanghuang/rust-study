use crate::mystructs::displayable::Coord;

pub fn do_it() {

    println!("\nIn demo_displayable::do_it()");
    
    let c = Coord::new(51.62, 3.94);
    println!("{}", c);
}