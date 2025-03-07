# Axum + Sqlite + Sqlx Demo

create .evn file
SQLx (the Rust database library) reads the DATABASE_URL environment variable from the .env file to know where the SQLite database is

```sh
cargo install sqlx-cli --features sqlite # Install SQLx CLI with sqlite support

sqlx database setup  # Runs migrations
```
