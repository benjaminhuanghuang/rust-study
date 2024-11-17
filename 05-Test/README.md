# Rust Test

## Run test

Rust run tests with muliti-threads

```sh
cargo test -- --help

cargo test -- --test-threads=4
cargo test -- --show-output

cargo test {test_function_name}

cargo test {test_function_name_matcher}
```



## Expected panics

```rust
#[should_panic(expected = "message")]
```

## Ignore test

```rust
#[ignore]
```
