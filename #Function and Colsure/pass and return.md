# Function

From: Rust Programming Part 2: Rust Advanced Concepts and Real-World Projects

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

You can pass reference parameters to a function, preceded by the & symbol.
The calling function retains ownership
The calling function can continue to use the value afterwards

The called function receives a reference
The called function borrows the value
When a function receives a reference parameter, uses * to deference, to obtain the underlying value

Rust allows a invoking method on a reference without explicit * dereferencing
Rust will automatically dereference when you invoke a method on a reference, so explicit dereferencing is optional.


If you declare a &str parameter: You can pass in a &String or &str
If you declare a &String parameter: You must pass in a &String

Displaying Reference Parameters
When you have a reference parameter:
• The default formatter {} displays the value
• The print formatter {:p} displays the address (i.e., pointer)

## Passing mutable reference

To pass a mutable reference parameter into a function:Precede the parameter with &mut

When a function receives a mutable reference parameter: Use * to deference, to obtain/modify the underlying value

## Returning a value

Returning a Copyable Value
• When you return a copyable value (e.g., 132), Rust bit-copies the value back to the caller
• When you return a non-copyable value (e.g., String): Rust moves ownership of the value back to the caller

## Returning a reference

You have to make ure the lifetime of the referred object whose lifetime is long enough.

Rust doesn't allow you to return a dangling reference
  • You can't return a reference to a local stack-based object

Return a string literal
```rs
fn some_func() -> &'static str {
    "Hello, World!"
}
```
Tell compiler the referred text is statically allocated. so it is valid for the lifetime of the program

## Returning a mutable reference

• Do comply with the borrow checker
• Don't return a dangling reference


## Lifetime management

The common way to ensure the lifetime of an object:
  • Pass an object by reference into a function
  • In the function, return a reference to the same object
