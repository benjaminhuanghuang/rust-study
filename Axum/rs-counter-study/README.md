# 【Rust 入门教程】从零开发 Rust 后端 API 服务 axum + sqlx + sqlite

https://www.bilibili.com/video/BV1GV411Q7e6

https://github.com/tokio-rs/axum/tree/main/examples

## Setup sqlx and sqlite

Create .evn file

```sh
brew install sqlite
cargo install sqlx-cli --features sqlite

sqlx database create
sqlx migrate add -r create_users
```

write migration script

```sh
sqlx migrate run
sqlx migrate revert
```
