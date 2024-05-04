pub fn do_it() {

    println!("\nIn demo_box1::do_it()");

    // Create a Box<i32> object, i.e. a simple smart pointer that points to data on the heap. 
    // Note the Box object itself lives on the stack.
    let boxed_number = Box::new(42);

    // println!() can dereference the Box explicitly or implicitly.
    println!("Explicitly dereferenced value: {}", *boxed_number);
    println!("Implicitly dereferenced value: {}", boxed_number);

    // If you want to use the value yourself, you must dereference explicitly.
    let value: i32 = *boxed_number;

    println!("Value: {}", value);
}