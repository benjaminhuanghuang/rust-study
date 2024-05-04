# Nested function

You can define a nested function
• i.e., a function nested inside an outer function
• Only visible within the outer function

• You must specify full Parameter types / return type info for a nested function

• A nested function cannot access variables in the outer scope
• You must pass any values you need as parameters into the function

```rust
pub fn do_it() {
  fn sqr(i: i32) -> i32 { 
    i * i 
  }

  println!("Square of 5 is {}", sqr(5));
  println!("Square of 7 is {}", sqr(7));
}
```
