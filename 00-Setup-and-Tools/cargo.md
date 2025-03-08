# Cargo (npm of rust)

```sh
cargo new <proj>
```

```sh
  cargo build   (create ./target)
  cargo build --release

  cargo run     (build and run)

  cargo check
```

## Cargo

cargo is Rust build tool, dependency manager, test runner and project bootstrapper

```sh
  cargo --version
```

Create new project

```sh
  cargo new hello-world
```

Build

```sh
  cargo build        # debug version
```

## Run

automatically recompile and run your Rust project whenever changes are detected in the src/ directory.

```sh
  cargo install cargo-watch

  cargo watch -q -c -w src/ -x run
```

-q (Quiet mode)

Suppresses unnecessary output from cargo watch, showing only essential information.
-c (Clear screen before running the command)

Clears the terminal before each run, making the output cleaner.
-w src/ (Watch the src/ directory only)

Tells cargo watch to monitor only the src/ folder for changes.
If omitted, it watches the entire project, including Cargo.toml.
-x run (Execute cargo run when a change is detected)

Runs cargo run every time a file in src/ is modified.

## Test

```sh
  cargo test

  cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```
