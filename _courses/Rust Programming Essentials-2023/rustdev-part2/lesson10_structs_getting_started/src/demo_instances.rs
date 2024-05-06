use crate::mytypes::Employee;

pub fn do_it() {    
    
    println!("\nIn demo_instances::do_it()");

    create_immutable_instance();
    create_mutable_instance();
}

fn create_immutable_instance() {

    let e1 = Employee {
        name: String::from("Jane"),
        salary: 1000,
        fulltime: true
    };

    println!("{} earns {}, fulltime status: {}", e1.name, e1.salary, e1.fulltime);
}

fn create_mutable_instance() {

    let mut e2 = Employee {
        name: String::from("John"),
        salary: 1000,
        fulltime: false
    };

    e2.salary *= 2;

    println!("{} earns {}, fulltime status: {}", e2.name, e2.salary, e2.fulltime);
}