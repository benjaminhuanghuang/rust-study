use crate::mytraits::log::Log;
use crate::mystructs::employee::Employee;

pub fn do_it() {

    println!("\nIn demo_trait_techniques::do_it()");

    let mut e1 = Employee::new(String::from("Mary"), 100, false);
    e1.payrise(100);
    
    e1.log();
    e1.log_verbose();
    println!("Btw Employee::LOG_TIMESTAMP is {}", Employee::LOG_TIMESTAMP);
}