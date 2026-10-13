# Code Your Own CLI With Rust

https://www.youtube.com/watch?v=5UA9UWWAagc

[中配](https://www.bilibili.com/video/BV1N4NizuE66?t=227.5&p=2)

## Setup

```sh
cargo new bestls
```

use clap(Command Line Argument Parser)

```toml
clap = { version = "4.5.39", features = ["derive"] }
```

## Run

```sh
cargo run -- -h
cargo run -- --version
```
