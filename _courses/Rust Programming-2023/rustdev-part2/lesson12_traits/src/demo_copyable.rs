use crate::mystructs::copyable::Currency;

pub fn do_it() {

    println!("\nIn demo_copyable::do_it()");
    
    let mut c1 = Currency::new(10, 99);

    // Copies (doesn't move), because Currency implements Copy.
    let mut c2 = c1;

    // Can still access c1.
    c1.dollars = 11;

    // Can also access c2.
    c2.dollars = 22;

    println!("c1: {:?}", c1);
    println!("c2: {:?}", c2);
}