use crate::mystructs::droppable::Place;

pub fn do_it() {

    println!("\nIn demo_droppable::do_it()");
    
    let _p1 = Place::new("Abertawe", "Cymru");

    if true {
        let _p2 = Place::new("Bordeaux", "France");
    }
    println!("Goodbye");
}