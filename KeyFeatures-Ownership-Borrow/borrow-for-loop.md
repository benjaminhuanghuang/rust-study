When you want to reuse container later in your program, use a reference.
when a reference is omitted, Rust assumes that container is no longer needed.
```
for item in &container {
// ...
}
```
If you need to modify each item during the loop, you can use a mutable reference by
including the mut keyword:
```
for item in &mut collection {
// ...
}
```

