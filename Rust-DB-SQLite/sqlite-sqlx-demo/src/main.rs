use dotenv::dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  dotenv().ok(); // Load .env file
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  let pool = SqlitePool::connect(&database_url).await?;
  sqlx::migrate!().run(&pool).await?;

  println!("Connected to SQLite database!");
  Ok(())
}
