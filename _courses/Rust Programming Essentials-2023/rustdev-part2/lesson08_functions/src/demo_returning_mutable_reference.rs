pub fn do_it() {
	
    println!("\nIn demo_returning_mutable_reference::do_it()");

	let mut s = String::from("hello");
	let r = some_func(&mut s);    // Receives mutable reference to a String.
	
	r.push_str(" and goodbye");
	println!("r: {}", r);
}

fn some_func(s: &mut String) -> &mut String {
	s.push_str(" world");
    s                   
}