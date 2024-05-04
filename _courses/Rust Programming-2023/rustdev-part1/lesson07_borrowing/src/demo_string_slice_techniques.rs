pub fn do_it() {

    println!("\nIn demo_string_slice_techniques::do_it()");

	slice_usage();
	slice_iteration();
	slice_part_of_string();
	slice_mutability();
}

fn slice_usage() {

	// Declare a string slice.
	let s1 = "hello😃";

	// Note: This example would also work with String objects.
	// let s1 = String::from("hello😃");

	println!("\ns1 ptr: {:p}, len: {}, text: {}", s1.as_ptr(), s1.len(), s1);
}

fn slice_iteration() {

	// Declare a string slice.
	let s2 = "hello😃";

	// Note: This example would also work with String objects.
	// let s2 = String::from("hello😃");

	println!("\nRaw bytes in s2 (in decimal, hex, and octal):");
	for b in s2.bytes() {
		println!("  {} {:x} {:o}", b, b, b);
	}

	println!("Characters in s2:");
	for ch in s2.chars() {
		println!("  {}", ch);
	}
}

fn slice_part_of_string() {

	let message = "howdy😎";

	// Create slices as a portion of string.
	let s3 = &message[0 .. 3];
	let s4 = &message[..3];
	let s5 = &message[2..5];
	let s6 = &message[2..];

	println!("\ns3 ptr: {:p}, len: {}, text: {}", s3.as_ptr(), s3.len(), s3);
	println!("s4 ptr: {:p}, len: {}, text: {}", s4.as_ptr(), s4.len(), s4);
	println!("s5 ptr: {:p}, len: {}, text: {}", s5.as_ptr(), s5.len(), s5);
	println!("s6 ptr: {:p}, len: {}, text: {}", s6.as_ptr(), s6.len(), s6);
}

fn slice_mutability() {

	let mut message = String::from("croeso");
	message.push_str(" o gymru");

	if true {
		let s: &mut str = &mut message[9..];
		s.make_ascii_uppercase();
	}

	println!("\nmessage: {}", message);
}