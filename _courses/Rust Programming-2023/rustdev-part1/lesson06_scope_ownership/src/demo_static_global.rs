use once_cell::sync::Lazy;   // Defined in the once_cell crate.
use chrono::{DateTime, Utc}; // Defined in the chrono crate. 

use std::thread::sleep;
use std::time::Duration;

pub static GLOBAL_MESSAGE: &str = "This is a global message";

static GLOBAL_TIMESTAMP: Lazy<DateTime<Utc>> = Lazy::new(|| {
	let now = Utc::now();
	println!("global GLOBAL_TIMESTAMP: {} ***** initialization *****", now.format("%T"));
	return now
});

pub fn do_it() {

    println!("\nIn demo_static_global::do_it()");

	f1();
	f1();
	f2();
	f2();
}

fn f1() {
	println!("\nIn f1, GLOBAL_MESSAGE:   {}", GLOBAL_MESSAGE);
	println!("In f1, GLOBAL_TIMESTAMP: {}", (*GLOBAL_TIMESTAMP).format("%T"));
}

fn f2() {
	println!("\nIn f2, GLOBAL_MESSAGE:   {}", GLOBAL_MESSAGE);
	println!("In f2, GLOBAL_TIMESTAMP: {}", (*GLOBAL_TIMESTAMP).format("%T"));
}