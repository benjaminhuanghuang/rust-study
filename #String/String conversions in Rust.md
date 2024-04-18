# String conversions in Rust

## Number to string

In Rust, you can use the to_string method to convert any type that implements the `ToString` trait to a String:
Any type that implements the `Display` trait also implements the `ToString` trait

```rust
let my_bool: bool = true;
let my_int: i32 = 23;
let my_float: f32 = 3.14;
let my_char: char = 'a';


println!("{}", my_bool.to_string());
println!("{}", my_int.to_string());
println!("{}", my_float.to_string());
println!("{}", my_char.to_string());
```

## string slice vs String

```rust
let  my_string = String::from("Hello world.");      // string slice to String
let my_string_slice = my_string.as_str();           // String to string slice

println!("{}", my_string);
println!("{}", my_string_slice);
```

## convert a string into a number type

convert a string into a number type or any other type that implements the FromStr trait.

```rust
let price = " 200,000 ";
println!("{}", price.trim().replace(",", "").parse::<u32>().unwrap());
```
