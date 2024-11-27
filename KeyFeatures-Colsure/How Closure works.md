# How Closures work under the covers

Rust convert a closure into a struct.

The struct has a `call` function, which is the closure body.

The struct contains fields, they are the captured variables.

The struct implements one or more traits FnOnce(called once), FnMut(state mutable), Fn(state immutably).

## A Closure that Implements EnOnce Only

Rust moves state into the closure:

```rust
fn outer_function () {
  let s1 = String:: from ("aaa") ;

  let closure = || {
    printin! ("{}", s1);
    std: : mem: : drop (s1)
  };
}
```

## A Closure that Implements EnOnce and FnMut

The closure capture the state mutable:

```rust
fn outer_function () {
  let s1 = String:: from ("aaa") ;

  let closure = || {
    s1.push_str ("bbb");
    printin! ("{}", s1);
  };
}
```

## A Closure that Implements EnOnce, FnMut, and Fn

The closure capture the state immutable:

```rust
fn outer_function () {
  let s1 = String:: from ("aaa") ;

  let closure = || {
    printin! ("{}", s1);
  };
}
```
