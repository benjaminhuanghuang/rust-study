use crate::mytypes::Employee;

pub fn do_it() {
        
    println!("\nIn demo_struct_return_value::do_it()");

    let e1 = build_employee(String::from("Jane"), 1000, true);
    print_employee(&e1);

    let mut e2 = build_employee_v2(String::from("John"), 1000, false);
    e2.salary += 750;
    print_employee(&e2);
}

fn build_employee(name: String, salary: u64, fulltime: bool) -> Employee {

    Employee {
        name: name,
        salary: salary,
        fulltime: fulltime
    }
}

fn build_employee_v2(name: String, salary: u64, fulltime: bool) -> Employee {
    Employee {name, salary, fulltime }
}

fn print_employee(emp: &Employee) {
    println!("{} earns {}, fulltime status: {}", emp.name, emp.salary, emp.fulltime);
}