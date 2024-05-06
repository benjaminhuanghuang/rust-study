#![allow(dead_code)]         // Allow dead code anywhere in this module.
#![allow (unused_imports)]   // Allow unused imports ("use" statements) anywhere in this module.

mod demo_locals;
mod demo_static_local;
mod demo_static_global;
mod demo_static_mutable;
mod demo_string_handling;
mod demo_copying_vs_moving;
mod demo_cloning;

use demo_static_global::GLOBAL_MESSAGE;

fn main() {

	// Uncomment one of these statements, to run a demo...

	// demo_locals::do_it();
	// demo_static_local::do_it();
	// demo_static_global::do_it();
	// demo_static_mutable::do_it();
	// demo_string_handling::do_it();	
	// demo_copying_vs_moving::do_it();
	// demo_cloning::do_it();

	// println!("\nBtw, GLOBAL_MESSAGE is {}", GLOBAL_MESSAGE);
}