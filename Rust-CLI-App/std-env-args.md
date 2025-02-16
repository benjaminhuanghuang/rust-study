# std::env::args

Rust provides std::env::args() to access command-line arguments.

```rust

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <your_name>", args[0]);
        return;
    }

    let name = &args[1];
    println!("Hello, {}!", name);
}
```
