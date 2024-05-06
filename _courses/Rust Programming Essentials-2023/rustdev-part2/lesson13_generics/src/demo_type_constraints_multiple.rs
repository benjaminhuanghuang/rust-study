use std::fmt::Debug;

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: i32
}

pub fn do_it() {

    println!("\nIn demo_type_constraints_multiple::do_it()");

    let p1 = Person {name: String::from("Andy"), age: 58};
    
    clone_and_print(&p1);
}

fn clone_and_print<T: Debug + Clone>(obj: &T) {
    let c = obj.clone();
    println!("{:?}", c);
}