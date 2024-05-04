use once_cell::sync::Lazy;   // Defined in the once_cell crate.
use chrono::{DateTime, Utc}; // Defined in the chrono crate. 

use std::thread::sleep;
use std::time::Duration;

pub fn do_it() {

    println!("\nIn demo_static_local::do_it()");

	static_init_at_compile_time();
	static_init_at_run_time();
}

fn static_init_at_compile_time() {
	static MESSAGE: &str = "Croeso o Gymru 😃";
	println!("MESSAGE: {}", MESSAGE);
}

fn static_init_at_run_time() {

	// This statement won't work. 
	// You can't initialize a static directly with a run-time value (it's not thread-safe).
	// static TIMESTAMP_WONT_WORK: DateTime<Utc> = Utc::now();

	println!("Curr time: {}", Utc::now().format("%T"));

	static TIMESTAMP: Lazy<DateTime<Utc>> = Lazy::new(|| {
		sleep(Duration::new(5, 0));
        let now = Utc::now();
		println!("Curr time: {}", now.format("%T"));
		return now
    });
	println!("TIMESTAMP: {}", (*TIMESTAMP).format("%T"));
}