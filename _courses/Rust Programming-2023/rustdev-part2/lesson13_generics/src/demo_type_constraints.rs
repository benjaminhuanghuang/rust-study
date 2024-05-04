pub fn do_it() {

    println!("\nIn demo_type_constraints::do_it()");

    let ai = [10, 20, 30, 40, 50];
    let af = [10.1, 20.2, 30.3];

    display_array(&ai);
    display_array(&af);
}

// This function won't compile.
// fn display_array_bad<T>(a: &[T]) {
//     for elem in a {
//         println!("{:?}", elem);
//     }
// }

fn display_array<T: std::fmt::Debug>(a: &[T]) {
    for elem in a {
        println!("{:?}", elem);
    }
}