pub fn do_it() {

    println!("\nIn demo_array_slice_intro::do_it()");

	let a = [10, 11, 12, 13, 14];

	// Declare an array slice (implicitly typed).
	let s1 = &a;

	// Declare an array slice (explicitly typed).
	let s2: &[i32] = &a;

	println!("s1 ptr: {:p}, len: {}, elements: {:?}", s1.as_ptr(), s1.len(), s1);
	println!("s2 ptr: {:p}, len: {}, elements: {:?}", s2.as_ptr(), s2.len(), s2);
}