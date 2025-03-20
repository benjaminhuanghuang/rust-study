# Unique pointer

We use Box::new(...) to allocate space on the heap and initialize that space with the supplied value

```rs
fn foo() {
  let x: Box<i32> = Box::new(75);

  println!("`x` points to {}", *x);

  let x = Box::new(75);
  let y = Box::new(42);
  // x = y;         // Not allowed, x is immutable.
  // *x = 43;       // Not allowed, *x is immutable.
  let mut x = Box::new(75);
  x = y;            // OK, x is mutable.
  *x = 43;          // OK, *x is mutable.
}
```

In Rust, there is no need to call free or delete

## Return

Unique pointers behave similarly to values - they are deleted when the variable goes out of scope

Owning pointers can be returned from a function and continue to live on

```rs
  fn foo() -> Box<i32> {
    let x = Box::new(75);
    x
  }

  fn bar() {
      let y = foo();
      // ... use y ...
  }
```

memory is initialised in foo, and returned to bar. x is returned from foo and stored in y, so it is not deleted. At the end of bar, y goes out of scope and so the memory is reclaimed.

## Owning pointers are unique

```rs
  fn foo() {
    let x = Box::new(75);
    let y = x;
    // x can no longer be accessed
    // let z = *x;   // Error.
  }


  fn bar(y: Box<isize>) {
  }

  fn foo() {
      let x = Box::new(75);
      bar(x);
      // x can no longer be accessed
      // let z = *x;   // Error.
  }
```

## Pointer to method

owning pointers must be dereferenced to use their values. However, method calls automatically dereference

```rs
  fn bar(x: Box<Foo>, y: Box<Box<Box<Box<Foo>>>>) {
    x.foo();
    y.foo();
  }
```

## Point to existed value

Calling Box::new() with an existing value does not take a reference to that value, it copies that value.

```rs
fn foo() {
    let x = 3;
    let mut y = Box::new(x);
    *y = 45;
    println!("x is still {}", x);
}
```

Primitive types have copy semantics, so in the above example the value 3 is copied, but for more complex values it would be moved
