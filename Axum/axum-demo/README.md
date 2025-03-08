# Readme

## Run server

```sh
cargo watch -q -c -w src/ -x "run --bin hello_axum"
```

## Run test

```sh
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```
