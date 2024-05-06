use crate::mystructs::cloneable::Flight;

pub fn do_it() {

    println!("\nIn demo_cloneable::do_it()");
    
    let f1 = Flight::new("LHR", "OSL");

    let mut f2 = f1.clone();
    f2.redirect_to("CPH");

    println!("f1: {:?}", f1);
    println!("f2: {:?}", f2);
}