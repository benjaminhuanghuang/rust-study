struct Employee {
    name: String,
    salary: u64 
}

pub fn do_it() {

    println!("\nIn demo_box2::do_it()");

    let boxed_emp = Box::new(Employee{name: String::from("Tom"), salary: 1000});
    
    // Pass the Box object into a function. 
    // This moves ownership of the Box into the function, because Box doesn't implement the Copy trait.
    process_employee(boxed_emp);

    // So we can't use the Box here (we've lost ownership).
    // println!("{} earns {}", boxed_emp.name, boxed_emp.salary);
}

fn process_employee(emp: Box<Employee>) {
    println!("{} earns {}", emp.name, emp.salary);
} // Box implements the Drop trait, so Box's drop() function is called here. It deallocates the item on the heap. 