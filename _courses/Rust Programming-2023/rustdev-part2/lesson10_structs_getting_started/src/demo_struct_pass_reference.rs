use crate::mytypes::Employee;

pub fn do_it() {
        
    println!("\nIn demo_struct_pass_reference::do_it()");

    let mut e1 = Employee {
        name: String::from("Jane"),
        salary: 1000,
        fulltime: true
    };
    
    // Pass struct by reference.
    print_employee(&e1);

    // Pass struct by mutable reference.
    reward_employee(&mut e1);
    print_employee(&e1);
}

fn print_employee(e: &Employee) {
    println!("Using explicit dereferencing: {} earns {}, fulltime status: {}", (*e).name, (*e).salary, (*e).fulltime);
    println!("Using implicit dereferencing: {} earns {}, fulltime status: {}", e.name, e.salary, e.fulltime);
}

fn reward_employee(e: &mut Employee) {
    
    // Using explicit dereferencing.
    (*e).salary += 500;

    // Using implicit dereferencing.
    e.salary += 250;
}