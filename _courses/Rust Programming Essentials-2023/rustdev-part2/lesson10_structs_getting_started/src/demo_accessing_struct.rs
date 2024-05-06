use crate::mytypes::Employee;

pub fn do_it() {    
    
    println!("\nIn demo_accessing_struct::do_it()");

    // Use fully-qualified struct name.
    let _e1: crate::mytypes::Employee;

    // Use alias.
    let _e2: Employee;

    let size = std::mem::size_of::<Employee>();
    println!("The size of Employee is {} bytes", size);
}