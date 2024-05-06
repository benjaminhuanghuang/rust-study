# Pass Closure into function

When you pass closures into functions, the closure specifies an algorithm the function can invoke

The function must specify what kind of closure (EnOnce, EnMut, or En) using type constraints

FnOnce(called once), FnMut(state mutable, called multiply times), Fn(state immutably).

## Function only calls a closure once

Pass colure implements EnOnce

```rust
fn receive_fnonce<F>(func: E) where F:EnOnce() {
  func () ;
}
```

## Function calls a closure many times, and you want to allow the closure to capture state mutably

You should bind the closure to FnMut

```rust
fn receive_fnmut<F>(func: F) where F:FnMut() {
  func() ;
  func()
}

```
