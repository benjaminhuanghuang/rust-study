use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;
use time::PrimitiveDateTime;

pub async fn establish_connection() -> Pool<Sqlite> {
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let pool = SqlitePool::connect(&db_url)
    .await
    .expect("Can't connect to database");

  pool
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
  pub id: i32,
  pub openid: String,
  pub session_key: String,
  pub created_at: PrimitiveDateTime,
  pub updated_at: PrimitiveDateTime,
}
