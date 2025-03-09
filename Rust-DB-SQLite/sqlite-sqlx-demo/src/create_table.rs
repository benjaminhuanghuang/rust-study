use dotenv::dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;

#[tokio::main]
async fn main() {
  dotenv().ok(); // Load .env file

  // Get database URL from environment
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  // Create database connection pool
  let pool = SqlitePool::connect(&database_url)
    .await
    .expect("Failed to connect to database");

  // Create the `users` table if it doesn't exist
  sqlx::query(
    "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );",
  )
  .execute(&pool)
  .await
  .expect("Failed to create table");
}
