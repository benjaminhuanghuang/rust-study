# Additional Rust Techniques

## use the value in Box

You can dereference the Box object explicitly or implicitly

```rust
printin!("{}", *boxed_number) ;

printin!("{}", boxed_number) ;
```

If you want to use the value in a Box, you must deference explicitly

```rust
  let value:i32 = *boxed_number ;   // unbox
```

Box does not implement the `Copy` trait, when you assign a Box object to another variable, the Box object is moved

```rust
  let value:i32 = *boxed_number ;   // unbox
  let value2 = value ;  // Error: value moved here
```

Pass Box to function will move the ownership of the Box to function, can not use Box later
```rust
let boxed_employee = Box::new(Employee::new("John", "Doe")) ;

process_employee(boxed_employee) ;  // boxed_employee moved, 
```

Box does implement the `Drop` trait. When a Box object goes out of scope, drop () is called drop () deallocates the heap-based storage
