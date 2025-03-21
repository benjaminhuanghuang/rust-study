# Music Platform Backend

## Tech Stack

- **Framework:** Axum (Rust)
- **Database:** SQLx (PostgreSQL)
- **Real-time:** WebSockets
- **Protocol:** HTTP/2
- **Validation:** Custom validation library

## Setup DB

```sh
docker run --name my_postgres -e POSTGRES_USER=myuser -e POSTGRES_PASSWORD=mypassword -e POSTGRES_DB=file_share -p 5432:5432 -d postgres
docker start my_postgres

cargo install sqlx-cli
sqlx database create
sqlx migrate run

cargo run
```

## database/favorites.rs
