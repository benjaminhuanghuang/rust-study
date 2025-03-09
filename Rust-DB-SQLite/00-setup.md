# Sqlx

## Install sqlx-cli

The SQLx CLI (sqlx-cli) is a command-line tool that helps manage databases when using SQLx in Rust.

```sh
# for sqlite
cargo install sqlx-cli --features sqlite # Install SQLx CLI with sqlite support

# for PostgreSQL
cargo install sqlx-cli --no-default-features --features postgres
```

## Config

Tells your application (such as SQLx) where to find the SQLite database file.

```toml
DATABASE_URL=sqlite:data.db
```

## Database management

```sh
# Creates the database if it does not exist, using the config in .env
sqlx database create
# Run all pending migrations from the migrations/ folder.
sqlx migrate run

# It is a shortcut that combines of sqlx database create and sqlx migrate run
sqlx database setup  # Runs migrations
```

## Migration
