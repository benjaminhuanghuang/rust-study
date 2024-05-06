use crate::mytypes::employee::Employee;

pub fn do_it() {

    println!("\nIn demo_associated_data::do_it()");

    let e1 = Employee::new(String::from("Matt"), 10_000, true);
    println!("{}", e1.to_string());

    let e2 = Employee::new(String::from("Mark"), 20_000, true);
    println!("{}", e2.to_string());

    let e3 = Employee::new(String::from("Luke"), 30_000, true);
    println!("{}", e3.to_string());

    let mut e4 = Employee::new(String::from("John"), 40_000, true);
    e4.payrise(500_000);
    println!("{}", e4.to_string());
}