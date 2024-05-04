use crate::mytraits::print::Print;
use crate::mystructs::point::Point;
use crate::mystructs::employee::Employee;

pub fn do_it() {

    println!("\nIn demo_inheritance_polymorphism::do_it()");

    let obj1 = Point::new(100, 200);
    let obj2 = Employee::new(String::from("Tom"), 1000, false);

    println!("Printable things:");
    print_something(&obj1);
    print_something(&obj2);

    let vec: Vec::<&dyn Print> = vec![&obj1, &obj2];
    println!("Printable things in a polymorphic collection:");
    for obj in vec {
        obj.print();
    }
}

fn print_something(p: &dyn Print) {
    p.print();
}