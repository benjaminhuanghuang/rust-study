pub fn do_it() {
	
    println!("\nIn demo_passing_values::do_it()");

	let n = 42;
	let s = String::from("hello");
	
	some_func(n, s);            // Copies n, but moves ownership of s.
	
	println!("n: {}", n);       // OK. We still own n.
	// println!("s: {}", s);    // NO! We don't own s.
}

fn some_func(iparam: i32, sparam: String) {
	println!("In some_func, iparam is {}", iparam);
	println!("In some_func, sparam is {}", sparam);
}