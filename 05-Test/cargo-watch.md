# Cargo watch

Install cargo-watch

```sh
cargo install cargo-watch
```

monitor folder src/

```sh
cargo watch -q -c -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture'
```
