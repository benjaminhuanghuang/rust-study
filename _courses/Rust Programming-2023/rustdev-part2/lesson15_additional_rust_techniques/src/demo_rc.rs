use std::rc::Rc;

struct Employee {
    name: String,
    salary: u64 
}

pub fn do_it() {

    println!("\nIn demo_rc::do_it()");

    let a = Rc::new(Employee{name: String::from("Emily"), salary: 1000});
    println!("Reference count initially is {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("Reference count after clone is {}", Rc::strong_count(&b));

    use_employee(&a);
    println!("Reference count after function is {}", Rc::strong_count(&a));

    if true {
        let d = Rc::clone(&a);
        println!("Reference count inside block is {}", Rc::strong_count(&d));
    }

    println!("Reference count after block is {}", Rc::strong_count(&a));

    // Note, Rust also supports non-owning "weak ref counts" which can be upgraded to "strong ref counts".
    // For details, see https://doc.rust-lang.org/std/rc/struct.Weak.html.
}

fn use_employee(rc_emp: &Rc<Employee>) {
    let c = Rc::clone(&rc_emp);
    println!("Reference count inside function is {}", Rc::strong_count(&c));
}