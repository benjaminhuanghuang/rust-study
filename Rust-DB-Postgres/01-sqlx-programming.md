# Sqlx programming

## Config

```toml
DATABASE_URL = postgres://<username>:<password>@<host>:<port>/<database_name>

DATABASE_URL = 'postgres://axum_postgres:laxum_postgres@127.0.0.1:5432/axum_postgres'
```

## Dependency

```sh
cargo add tokio sqlx serde serde_json uuid
```

```toml

[dependencies]
sqlx = {version="0.8.3", features=["runtime-tokio","tls-rustls", "prostgres","time"]}
# sqlx is an asynchronous library
tokio={version="1.0", features=["full"]}
# sqlx will read .env file
dotenv ="0.15"

```
