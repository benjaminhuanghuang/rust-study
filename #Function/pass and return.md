# Function

## Passing parameters by value

```rs
fn f1(iparam: i32, sparam: String) {
    println!("x = {}, y = {}", iparam, sparam);

    // Drop function for String parameter is called when the function goes out of scope to deallocate the memory.
}

fn higher_level_func() {
  let n = 10;
  let s = String::from("hello");
  f1(n, s);

  println!("n = {}", n); // Still own n.
  println!("s = {}", s); // Error: Don't won s, can not use it anymore.
}
```

i32 implements the Copy trait, so it is copied and passed to function.
String does not implement the Copy trait, the ownership is moved to the function.

When you pass a copyable value (e.g., i32):
• Rust bit-copies the value into the parameter
• The original function retains ownership of the value

When you pass a non-copyable value (e.g., String):
• Rust moves ownership of the value into the parameter
• The original function loses ownership of the value!

## Passing parameters by reference
