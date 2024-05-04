pub fn do_it() {

    println!("\nIn demo_cloning::do_it()");

	// Simple types implement the Copy trait
	let a = 42;
	let b = a;
	println!("a: {}, b: {}", a, b);

	// Other types don't implement the Copy trait.
	// If you do want to copy without invalidating the original, call clone().
	let mut s1 = String::from("hello");
	let s2 = s1.clone();

	s1.push_str(" world, det er istid eller kaffetid snart");
	println!("s1: {}", s1);
	println!("s2: {}", s2);
}