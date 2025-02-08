## Run test in multi threads

```sh
  cargo test -- --test-threads=10
```

## Only run integration test

tests in /tests folder

```sh
  cargo test --test '*'
```

## Test utils

Since Rust treats every source file at the top level of the test directory as an integration test, this test utilities file will also get compiled into a test binary and run and show up in the test output. Even if it doesn't actually contain any annotated test functions.

The workaround to prevent that from happening is to use the alternate structure for modules moving the code into a file named mod.rs within a sub directory with the module test_utils.

## Can NOT write integration tests for Binary crate

only library crates expose their functionality for other crates to import and use.

Binary crates are intended to run on their own and cannot be used by other crates.

Therefore, we cannot write integration tests to directly test a binary crate.
