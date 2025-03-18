# Box

Box is a smart pointer with a `fixed size` which places its value `on the heap` rather than the stack

```rs
  let b = Box::new(5);
  println!("b = {}", b);
```

```
type TestResult = Result<(), Box<dyn std::error::Error>>;
```

Box indicates that the error will live inside a pointer where the memory is dynamically
allocated on the heap rather than the stack,

dyn indicates that the method calls on the std::error::Error trait are dynamically dispatched.

If a variable doesn’t have a fixed, known size, then Rust can’t store it
on the stack. The solution is to instead allocate memory on
the heap by putting the return value into a Box, which is a pointer with a known size.

```
  fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {

  }
```

To get data from a Box in Rust, you can use the .as_ref() or .as_mut() method to convert the Box into a reference, then access the data using the reference. For example:

```
let mut box_data = Box::new(5);

// Convert the Box into a mutable reference using .as_mut()
let data = box_data.as_mut();

// Access the data using the reference
println!("Data from Box: {}", *data);
```
