use crate::mytypes::Employee;

pub fn do_it() {
        
    println!("\nIn demo_struct_return_reference::do_it()");

    let mut e1 = build_employee(String::from("Jane"), 1001, true);
    let mut e2 = build_employee(String::from("John"), 1000, false);

    let ri = choose_employee(&e1, &e2);
    print_employee(ri);

    let rm = choose_mut_employee(&mut e1, &mut e2);
    rm.salary *= 2;
    print_employee(rm);
}

fn build_employee(name: String, salary: u64, fulltime: bool) -> Employee {
    Employee {name, salary, fulltime}
}

fn choose_employee<'a>(e1: &'a Employee, e2: &'a Employee) -> &'a Employee {
    if e1.salary > e2.salary {e1} else {e2}
}

fn choose_mut_employee<'a>(e1: &'a mut Employee, e2: &'a mut Employee) -> &'a mut Employee {
    if e1.salary > e2.salary {e1} else {e2}
}

fn print_employee(e: &Employee) {
    println!("{} earns {}, fulltime status: {}", e.name, e.salary, e.fulltime);
}