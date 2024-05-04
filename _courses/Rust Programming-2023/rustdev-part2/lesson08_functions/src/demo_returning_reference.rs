pub fn do_it() {
	
    println!("\nIn demo_returning_reference::do_it()");

	let s = String::from("hello world");
	
	let r1 = get_first_word(&s);        // Receives &str (implicit typing).
	println!("r1: {}", r1);

	let r2: &str = get_first_word(&s);  // Receives &str (explicit typing).
	println!("r2: {}", r2);

	let message: &str = get_message(99);
	println!("message: {}", message);
}

fn get_first_word(s: &str) -> &str {
	let mut pos = 0;
	for ch in s.chars() {
		if ch == ' ' {
			break;
		} 
		pos += 1
	}           
	&s[..pos]
}

fn get_message(mark: i32) -> &'static str {
	if mark >= 50 {"PASS😃"} else {"FAIL😢"}
}
 
// This won't compile, because it returns a dangling reference.
/*
fn bad_func_1() -> &str {
   let s = String::from("hello");
   &s                   
}
*/

// This won't compile either, because it also returns a dangling reference.
/*
fn bad_func_2(s: String) -> &str {
	let mut pos = 0;
	for ch in s.chars() {
		if ch == ' ' {
			break;
		} 
		pos += 1
	}           
	&s[..pos]
}
*/