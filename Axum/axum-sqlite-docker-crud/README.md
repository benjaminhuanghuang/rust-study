# Mastering Rust Web Services: From Axum to CRUD Operations

https://www.youtube.com/watch?v=JUWSy9pXgMQ
https://github.com/thebracket/webinar_axumcrud

## SQLite

```sh
cargo install sqlx-cli
```

Create a .env to tell sqlx where is the database

```yaml
DATABASE_URL="sqlite::memory:"
```

Create a migration for database

```sh
sqlx migrate add init
```
