use crate::mytraits::collections::{Queue, Deque};
use crate::mystructs::dequeimpl::MyDeque;

pub fn do_it() {

    println!("\nIn demo_trait_inheritance::do_it()");
    
    let mut d = MyDeque::new();

    d.push_back(300);
    d.push_back(400);
    d.push_back(500);
    d.push_front(200);
    d.push_front(100);

    println!("MyDeque object has {} items:", d.len());
    loop {
        match d.pop_back() {
            Some(v) => println!("  {}", v),
            None    => break
        }
    }
}