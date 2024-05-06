pub fn do_it() {

    println!("\nIn demo_generic_functions::do_it()");

    // Call a function hard-coded to receive an array of i32.
    let ai = [10, 20, 30, 40, 50];
    process_array_ints(&ai);

    // Call a function hard-coded to receive an array of f64.
    let af = [10.1, 20.2, 30.3];
    process_array_floats(&af);

    // Call a generic function, specifying the type parameter explicitly.
    process_array::<i32>(&ai);
    process_array::<f64>(&af);

    // Call a generic function, relying on type inference for the type parameter.
    process_array(&ai);
    process_array(&af);
}

fn process_array_ints(arr: &[i32]) {
    println!("{} elements, {} bytes each", 
              arr.len(), 
              std::mem::size_of::<i32>());
}

fn process_array_floats(arr: &[f64]) {    
    println!("{} elements, {} bytes each", 
              arr.len(), 
              std::mem::size_of::<f64>());
}

fn process_array<T>(arr: &[T]) {   
    println!("{} elements, {} bytes each", 
              arr.len(), 
              std::mem::size_of::<T>());
}