pub fn do_it() {
	
    println!("\nIn demo_passing_references::do_it()");

	let n = 42;
    let s = String::from("Ola Nordmann");
	
	some_func1(&n, &s);       // Borrows n and s, i.e. passes references.
	// some_func2(&n, "Jo");  // NO! Can't pass &str into &String.
	
	some_func2(&n, &s);
	some_func2(&n, "Siv Nordmann");

	some_func3(&n, &s);       
	some_func3(&n, "Per Nordmann");       

	println!("n: {}", n);     // OK. We still own s.
	println!("s: {}", s);     // OK. We still own n.
}

fn some_func1(iparam: &i32, sparam: &String) {
	if *iparam >= 50 {
		println!("{}, {}, PASS 😃", *iparam, (*sparam).to_uppercase());
	}
	else {
		println!("{}, {}, FAIL 😢", *iparam, (*sparam).to_lowercase());
	}

	// Note Rust allows you to simplify method calls. Both the following are OK:
	//   (*sparam).to_uppercase()
	//   sparam.to_uppercase()
}

fn some_func2(iparam: &i32, sparam: &str) {
	if *iparam >= 50 {
		println!("{}, {}, PASS 😃", *iparam, sparam.to_uppercase());
	}
	else {
		println!("{}, {}, FAIL 😢", *iparam, sparam.to_lowercase());
	}
}

fn some_func3(iparam: &i32, sparam: &str) {
	println!("Values {0} and {1}, addresses {0:p} and {1:p}", iparam, sparam);
}