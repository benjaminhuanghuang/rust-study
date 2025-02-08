# Rust unit test boilerplate t

```rs
// This attribute tells the Rust compiler to only compile this module when running tests.
#[cfg(test)]
mod tests { //Defines a test module named tests
    // Brings all the items from the parent module into scope.
    use super::*;

    #[test] // Marks the function as a unit test. The test runner will execute this function when you run cargo test.
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[ignore] // Ignore the test
    fn test_bad_add() {
         assert_eq!(bad_add(1, 2), 3);
    }
}
```

## Test skeleton

```rs
  // -- FIXTURE

  // -- ACTION

  // -- CHECK - deleted item
```
