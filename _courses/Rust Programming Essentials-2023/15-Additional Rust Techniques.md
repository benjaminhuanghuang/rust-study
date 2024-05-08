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

## Using Box

A Box points to an object on the heap.
The Box object is stored on the stack. When a Box goes out of scope, the head object is deallocated.

Define a recursive data structure with Box

```rust
struct NodeBad {
    data: i32,
    next: Option<NodeBad>,
}
```

whenever you define a structure, the compiler needs to know how big it is in bytes, so we can know how much space to allocate on the stack.

```rust
struct NodeBad {
    data: i32,
    next: Option<Box<NodeBad>>,
}
```

## Rc

Rc allocates objects on the heap. Maintains a reference count.
When an Rc out of scope, the reference count is decremented, when the reference count goes to zero, the object is deallocated.
