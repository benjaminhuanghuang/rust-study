pub fn do_it() {

    println!("\nIn demo_borrow_checker::do_it()");

	defining_many_immutable_references();
	restrictions_after_defining_mutable_reference();
	restrictions_after_defining_immutable_reference();
}

fn defining_many_immutable_references() {
	
	let s = String::from("hello");

	// You can define any number of immutable borrows.
	let r1 = &s;		
	let r2 = &s;		
	let r3 = &s;		

	println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
}

fn restrictions_after_defining_mutable_reference() {
	
	let mut s = String::from("huey");
	s.push_str(" louie");

	// If you have a mutable borrow...
	let r1 = &mut s;

	// Then you can't have any other borrows at all.
	// let r2 = &mut s;       // Nope!
	// let r3 = &s;           // Nope!

	// And you can't do this either (println! tries to borrow s).
	// println!("s: {}", s);  // Nope!

	r1.push_str(" dewey");

	println!("r1: {}", r1);
}

fn restrictions_after_defining_immutable_reference() {
	
	let mut s = String::from("huey");
	s.push_str(" louie");
	
	// If you have immutable borrow(s)...
	let r1 = &s;
	let r2 = &s;

	// Then you can't have any immutable borrows.
	// let r3 = &mut s;       // Nope!
	
	// And you also can't modify the original object either.
	// s.push_str(" dewey");  // Nope!
	
	println!("s: {}, r1: {}, r2: {}", s, r1, r2);
}