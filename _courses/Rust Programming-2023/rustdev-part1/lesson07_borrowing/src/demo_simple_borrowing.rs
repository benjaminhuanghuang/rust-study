pub fn do_it() {

    println!("\nIn demo_simple_borrowing::do_it()");

	implicitly_typed_borrowing();
	explicitly_typed_borrowing();
	mutable_borrowing();
}

fn implicitly_typed_borrowing() {
	
	let s1 = String::from("huey");
	let r1 = &s1;
	
	println!("Implicitly typed borrowing, s1: {}, r1: {}", s1, r1);
}

fn explicitly_typed_borrowing() {
	
	let s2: String = String::from("louie");
	let r2: &String = &s2;
	
	println!("Explicitly typed borrowing, s2: {}, r2: {}", s2, r2);
}

fn mutable_borrowing() {
	
	let mut s3: String = String::from("dewey");
	let r3: &mut String = &mut s3;

	r3.push_str(" duck");
	println!("Mutable borrowing, r3: {}", r3);
}