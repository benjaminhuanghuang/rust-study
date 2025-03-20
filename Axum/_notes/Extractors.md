State<T> – Extract Shared Application State

```rs
use axum::{extract::State, routing::get, Router};
use sqlx::PgPool;

async fn handler(State(db): State<PgPool>) {
    // Use `db` to interact with PostgreSQL
}

```
