# Build a CRUD REST API with Rust Axum | Tutorial

https://www.youtube.com/watch?v=NJsTgmayHZY
https://github.com/cudidotdev/Rust-axum-postgres-CRUD-app

## Setup

Create Postgres db: axum_postgres

```sh
sqlx migrate add init


# Modify 20250309233633_init.sql
CREATE TABLE tasks(
  task_id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  priority INT
)

sqlx migrate run
```

## Dependencies

```toml
[dependencies]
#axum
axum = "0.7.4"
tokio = { version = "1.35.1", features = ["full"] }

#postgres
sqlx = {version = "0.7.3", features = ["runtime-tokio", "tls-native-tls", "postgres", "macros"]}

#serde
serde = { version = "1.0.195", features = ["derive"] }
serde_json = {version = "1.0.111"}

#env
dotenvy = "0.15.7"
```

## Config

.env

## API
