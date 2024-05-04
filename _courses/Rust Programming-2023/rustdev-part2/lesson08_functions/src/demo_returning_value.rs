pub fn do_it() {
	
    println!("\nIn demo_returning_value::do_it()");

	let n = func_returning_copyable_type();	     // Receives copy of value.
	println!("n: {}", n);

	let s = func_returning_non_copyable_type();	 // Receives ownership of value.
	println!("s: {}", s);
}	

fn func_returning_copyable_type() -> i32 {
	let n = 42;
	return n;       // Copies value back to caller.
}

fn func_returning_non_copyable_type() -> String {
	let s = String::from("hello");
	return s;       // Moves ownership back to caller.
}

// This function doesn't compile. 
// It should return a String object.
// Instead it returns a string literal (i.e. &str).
/*
fn bad_func_returning_string_literal(mark: i32) -> String {
 	return if mark >= 50 {"PASS"} else {"FAIL"};
}
*/