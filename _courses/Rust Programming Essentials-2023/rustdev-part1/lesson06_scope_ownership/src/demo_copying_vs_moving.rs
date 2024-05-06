pub fn do_it() {
	
    println!("\nIn demo_copying_vs_moving::do_it()");
    
	// Simple types implement the Copy trait. 
	// When you assign, it bit-copies the value.
	let a = 42;
	let b = a;
	println!("a: {}, b: {}", a, b);
	
	// Other types don't implement the Copy trait. 
	// When you assign, it moves the value (i.e. transfers ownership). 
	// The original variable is invalidated.
	let s1 = String::from("hello");
	let s2 = s1;
	
	// Nope! Can't use s1 because its value has been moved into s2.
	// println!("s1: {}", s1);

	// This is ok.
	println!("s2: {}", s2);
}  