
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

-p profile
-c config file
-e extra args

```
  cargo run -- run -p rust -c fixtures\test.yml -e a=100 -e %c=3 -e @m=3

  ./xdiff.exe run -p rust -c ..\..\fixtures\test.yml -e a=100 -e %c=3 -e @m=3
```


## Rust 项目实操 - xdiff (3)：实现核心逻辑
https://www.youtube.com/watch?v=1iJzZjxCQB8&list=PL2XM89iiOzksOyZsNssA2-WA8N2eZ46wG&index=3

Move RequestProfile into req.rs



## Rust 项目实操 - xdiff (4)：提供更友好的错误提示
https://www.youtube.com/watch?v=e5mitBpoIlQ&list=PL2XM89iiOzksOyZsNssA2-WA8N2eZ46wG&index=4

Add DiffConfig.validate


## Rust 项目实操 - xdiff (5)：交互式生成配置
https://www.youtube.com/watch?v=TnIeSL74TeA&list=PL2XM89iiOzksOyZsNssA2-WA8N2eZ46wG&index=5

Add sub command: Parse
```
  cargo add dialoguer
```

## Rust 项目实操 - xdiff (6)：语法高亮以及支持 xreq
https://www.youtube.com/watch?v=rWi1fdcVM4g&list=PL2XM89iiOzksOyZsNssA2-WA8N2eZ46wG&index=6

语法高亮
```
cargo add syntect
```

support xreq
```
  xreq -p todo -e @title="hello"

  cargo run --bin xreq -- run -p todo -c fixtures\xreq_test.yml -e a=100 -e %c=3 -e @m=3
```

Cargo.toml
```
[[bin]]
name = "xdiff"
path = "src/bin/xdiff.rs"

[[bin]]
name = "xreq"
path = "src/bin/xreq.rs"
```

## Rust 项目实操 - xdiff (7)：支持管道和重定向
https://www.youtube.com/watch?v=DVjNglWn8rk&list=PL2XM89iiOzksOyZsNssA2-WA8N2eZ46wG&index=7


