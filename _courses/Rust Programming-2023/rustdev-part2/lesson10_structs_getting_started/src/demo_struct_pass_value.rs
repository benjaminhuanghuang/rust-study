use crate::mytypes::Employee;

pub fn do_it() {
        
    println!("\nIn demo_struct_pass_value::do_it()");

    let e1 = Employee {
        name: String::from("John"),
        salary: 1000,
        fulltime: false
    };
    
    // Pass struct by value (and move ownership into the function).
    consume_employee(e1);

    // Can't use e1 now...
    // println!("{}", e1.name);
}

fn consume_employee(e: Employee) {
    
    println!("{} earns {}, fulltime status: {}", e.name, e.salary, e.fulltime);

}   // Employee object dropped here.