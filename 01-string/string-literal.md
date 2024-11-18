# Where Is a String Literal Stored?

In Rust, a string literal (e.g., "abc") is stored in the read-only memory (static memory) of the program's binary

A string literal has the type &str, which is a reference to a statically allocated string slice.

The actual string data ("abc") is stored in the read-only section of the program's binary, often referred to as the .rodata segment.
This ensures the string literal is immutable and cannot be modified.

String literals have a 'static lifetime, meaning they exist for the entire duration of the program.

```rs
// The string "abc" is stored in the binary's read-only memory.
// The variable s is a reference to this memory.
let s1 = "abc"; // The full type signature is &'static str.

// To make a string mutable or store it on the heap, convert it into a String.
let s2 = String::from("abc");
```
