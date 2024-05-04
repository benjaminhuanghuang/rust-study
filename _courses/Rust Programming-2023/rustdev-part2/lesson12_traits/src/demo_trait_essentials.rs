use crate::mytraits::print::Print;
use crate::mystructs::employee::Employee;

pub fn do_it() {

    println!("\nIn demo_trait_essentials::do_it()");
    
    let mut e1 = Employee::new(String::from("John"), 100, false);
    e1.payrise(100);
    
    e1.print();
}