## Box
Box is a smart pointer with a `fixed size` which places its value `on the heap` rather than the stack


```
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