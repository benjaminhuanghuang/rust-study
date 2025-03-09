# Readme

## Run server

```sh
cargo watch -q -c -w src/ -x "run --bin hello_axum"
```

-q (Quiet mode)

Suppresses extra output from cargo watch, making logs cleaner.

-c (Clear screen before running the command)

Clears the terminal before each execution, keeping output fresh.

-w src/ (Watch the src/ directory only)

Ensures cargo watch monitors changes only inside the src/ folder, ignoring other files like Cargo.toml or tests/.

-x "run --bin hello_axum" (Execute cargo run --bin hello_axum)

-x specifies a command to execute.
run --bin hello_axum runs the specific binary named hello_axum instead of the default main.rs.

## Run test

```sh
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

-q (Quiet mode)

Suppresses unnecessary output, showing only essential information.
-c (Clear screen before running the command)

Clears the terminal before each execution for a clean output.
-w tests/ (Watch the tests/ directory)

Watches for changes only inside the tests/ folder.
Useful for integration tests in the tests/ directory.
-x "test -q quick_dev -- --nocapture" (Execute cargo test for quick_dev with --nocapture)

test → Runs Rust tests.
-q → Quiet mode, reducing unnecessary output from Cargo.
quick_dev → Runs only the test or module named quick_dev instead of all tests.
-- --nocapture → Ensures that printed output (println!) inside tests is displayed in the terminal.
