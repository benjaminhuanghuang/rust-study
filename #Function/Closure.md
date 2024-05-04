# Closure

• Closures in Rust are the equivalent of lambdas in other programming languages
• A closure is similar to a nested function, but more flexible in several respects:
• A closure can infer parameter/return types
• A closure can capture external variables

If a closure in Rust intends to modify the values of captured external variables, it must be defined as mutable. This ensures that the closure has the appropriate permissions to change the external state.


## Closure Syntax

• Define parameters inside ||
• Optionally specify parameter/return types
• Define the closure body inside { }
• Last expression in the closure is the return value

```rust
let get_timestamp = || => DateTime<Utc> {
  Utc::now()
};

let c = |x: i32, y:i32| -> i32 {
  x + y
};
```

## Type inference with closure

• Rust will infer the parameter types, based on the values you pass in
• Rust will infer the return type, based on the value returned

```rust
let reciprocal = |n| if n = 0.0 {0.0} else {1.0 / n};   // type can be inferred by the value passed in when the closure is called
```

Once you've invoked the closure and the compiler has inferred a type for its parameters, you have to be consistent with the type. 
Calling it again with a different type will result in an error due to type mismatch. 
Rust doesn't regenerate the closure for each call, and the type is decided based on the first invocation.

## Capturing external variables by reference

A closure can access variables defined in outer scope
This is known as capturing

```rust
fn outer_function () {
  // Declare some variables here.
  let some_closure = |params| {
    //Access captured variables here.
    ...
  }
};
```

The Rust compiler decides how to capture a variable:
• Capture an immutable reference
• Capture a mutable reference
• Capture the value (move ownership)
Rust makes this decision based on how the closure uses a variable

## Capturing external variables by value

```rust
let message = String:: from ("hello");
...
let consume_ message = || {
  println! ("Message in closure: {}", message) ;
  std::mem::drop(message) ;  // request the ownership of message
} ;
// Can't use 'message' here, it's owned by closure!
```

You can force a closure to capture variables by value
• Prefix the closure with the move keyword

This is useful when you spawn another thread
• The thread executes the code in a closure
• The thread might outlive the outer function
• So the closure must capture variables by value

```rust
let message = String::from("hello");

std::thread::spawn(move || {
  println!("Message in closure: {}", message);
 std::thread::sleep(Duration::new(5, 0));
});
```
