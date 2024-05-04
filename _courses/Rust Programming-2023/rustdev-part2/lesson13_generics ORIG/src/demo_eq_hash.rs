use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
struct EmpCode {
    country: String,
    empnum: String
}

impl EmpCode {
    fn new(c: &str, e: &str) -> EmpCode {
        EmpCode {
            country: c.to_string(), 
            empnum: e.to_string()
        }
    }
}

#[derive(Debug)]
struct Emp {
    name: String,
    salary: f32
}

impl Emp {
    fn new(n: &str, s: f32) -> Emp {
        Emp {
            name: n.to_string(), 
            salary: s
        }
    }
}

pub fn do_it() {

    println!("\nIn demo_eq_hash::do_it()");

    let mut staff: HashMap<EmpCode, Emp> = HashMap::new();

    staff.insert(
        EmpCode::new("UK", "001"),
        Emp::new("Matt", 1000.0)
    );

    staff.insert(
        EmpCode::new("UK", "002"),
        Emp::new("Mark", 2000.0)
    );

    staff.insert(
        EmpCode::new("US", "001"),
        Emp::new("Mary", 3000.0)
    );


    let emp = &staff[&EmpCode::new("UK", "002")];
    println!("{:?}", emp)
}