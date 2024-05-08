# Generic in Rust

Can be used with struct, function, methods.

## Generic function

The T should implement std::cmp::PartialOrd

```
  fn get_biggest<T: PartialOrd>(a:T, b:T) ->T {

  }
```
