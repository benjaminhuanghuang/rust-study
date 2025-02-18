# #[derive(Debug)]

In Rust, #[derive(Debug)] is an attribute that automatically implements the Debug trait for a struct or enum.
The Debug trait allows you to format and print the internal state of an object using the {:?} or {:#?} format specifier
with println!().

## {:?} or {:#?}

Both {:?} and {:#?} are used to print debug information for structs, enums, and other types that implement the Debug trait in Rust.

```rs
let p = Person {
    name: "Alice".to_string(),
    age: 30,
};

// Compact Debug
println!("{:?}", p);

// Pretty Debug
println!("{:#?}", p);
```
