pub fn do_it() {

    println!("\nIn demo_string_slice_intro::do_it()");

	slice_string_object();
	slice_string_literal();
}

fn slice_string_object() {

	let obj = String::from("hello");
	
	// Define a &String variable (implicitly typed).
	let s1 = &obj;

	// Define a &str variable (explicitly typed).
	let s2: &str = &obj;

	println!("s1: {}, s2: {}", s1, s2);
}

fn slice_string_literal() {

	// Define a &str variable (implicitly typed).
	let s3 = "hello";

	// Define a &str variable (implicitly typed)
	let s4: &'static str = "world";

	println!("s3: {}, s4: {}", s3, s4);
}