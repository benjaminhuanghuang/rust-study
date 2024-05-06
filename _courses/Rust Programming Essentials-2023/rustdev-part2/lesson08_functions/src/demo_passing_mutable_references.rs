pub fn do_it() {
	
    println!("\nIn demo_passing_mutable_references::do_it()");

	let mut n = 42;
    let mut s = String::from("hello");
	
	some_func(&mut n, &mut s);	 // Mutably borrows n and s.
	
	n += 1_000_000;              // OK. We still own n.
	s.push_str("👍👍👍");      // OK. We still own s.

	println!("n: {}", n);       
	println!("s: {}", s);
}

fn some_func(iparam: &mut i32, sparam: &mut String) {
	println!("Values initially: {}, {}", iparam, sparam);
	*iparam += 10;  
	sparam.push_str(" world");
    println!("Values afterward: {}, {}", iparam, sparam);
}