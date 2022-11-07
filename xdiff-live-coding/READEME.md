
https://www.youtube.com/watch?v=lsFBnfnY_EM&list=PL2XM89iiOzksOyZsNssA2-WA8N2eZ46wG

https://github.com/tyrchen/rust-training/tree/master/live_coding/xdiff-live

## Rust 项目实操 - xdiff (1)：基本思路和数据结构

- clap for command line
  ```
    cargo add clap --features derive
  ```
- yaml for config
```
  cargo add serde_yaml
```

- Serialize http method
```
  cargo add http_serde
  cargo add url --features serde 
```
- tokio
```
  cargo add tokio --features full
```

- anyhow 
```
  cargo add anyhow
```
- reqwest 
  
  reqwest use native tls by default
```
  cargo add reqwest --features rustls --no-default-features
```

- compare and color output
https://github.com/mitsuhiko/similar
```
  cargo add similar
```

- Run example
```
  cargo run --example similar.rs
  cargo run --example config
```


## Rust 项目实操 - xdiff (2)：使用 clap 构建 CLI
https://www.youtube.com/watch?v=kWzR9CMS47k&t=61s



## Rust 项目实操 - xdiff (3)：实现核心逻辑
