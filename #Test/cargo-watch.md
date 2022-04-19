## Cargo watch
Install cargo-watch
```
cargo install cargo-watch
```

moniter folder src/
```
cargo watch -q -c -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture'
```

