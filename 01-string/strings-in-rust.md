# Strings in Rust

In Rust, there are two primary string types:

- String (owned string)
  The String type is a heap-allocated, growable, and mutable string type.
  It is owned by the variable that holds it, meaning it can be modified and resized at runtime.

- &str (string slice)
  A &str is a borrowed string slice that points to a sequence of characters in memory. It is typically used when you want to reference part of a String or a string literal. It cannot be mutated because it's a reference to immutable data.

```rs
let mut s = String::from("Hello");
s.push_str(", world!"); // You can mutate the String by adding more data
println!("{}", s); // Output: Hello, world!

// string literal stored in the binary.
let s: &str = "Hello, world!";
println!("{}", s); // Output: Hello, world!
```

- str
  In Rust, str is a primitive string type that represents a sequence of UTF-8 encoded characters.
  str is an unsized type, which means it doesn’t have a fixed size at compile time.
  unsized types like str cannot be directly instantiated as variables.

  Rust’s ownership system relies heavily on borrowing, and str enables this by being an unowned type that can be borrowed via &str
