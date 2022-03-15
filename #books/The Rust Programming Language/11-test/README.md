# Rust Test

## Unit Test
  - write tests for the functions inside the program
  - can test private 
```
#[cfg(test)]
mod tests {
  #[test]
  fn it_works(){
    assert_eq!(4, 2 + 2)
  }

}

cargo test
```


## Integration Test
  - Outside-in, tests programs as the user might
  - `tests` directory parallel to the `src` directory, and file tests/test.rs
  - tests 目录会被特殊处理， 目录下每个测试文件都是一个单独的crate
  - #[test] attribute tells Rust to run this function when testing.


```
  use <lib>

  #[test]
  


  carto test <function>
  carto test --test <file>
```
## Vefirfy
```
  assert!(true);

  assert!(res.is_ok());

  assert_eq!

  result.assert().success();

```


## Run test
run a binary target
```
  cargo run --quiet --bin <name>
```

run the tests in order by using a single thread
```
cargo test -- --test-threads=1.
```



