
## Use panic macro to quit program

program will crash with a message

Program unwinds and cleans up the stack

Used for unrecoverable state

```
fn guess(n: i32) -> bool {
    if n < 1 || n > 10 {
        panic!("Invalid number: {}", n);
    }
    n == 5
}
```

Rust emits a panic during execution
```
  lev v = vec![1, 2, 3];

  println!("{}", v[6]);  // this will cuase a panic
```