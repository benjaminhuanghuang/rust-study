/*

Debug Formatting (:?)


{a:?} means print a in debug mode.

This is useful when a is a complex type like a struct, tuple, or array.

The :? tells Rust to use the Debug trait for formatting.
*/

println!("a: {a:?}");

/*
Adding #, eg {a:#?}, invokes a “pretty printing” format, which can be easier to read.
*/
