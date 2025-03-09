use sqlx::{sqlite::SqlitePool, Error, FromRow};

#[derive(Debug, FromRow)]
struct User {
  id: i64,
  name: String,
  email: String,
}

// Create user
async fn create_user(pool: &SqlitePool, name: &str, email: &str) -> Result<User, Error> {
  let query = "INSERT INTO users (name, email) VALUES (?, ?)";
  let row = sqlx::query(query)
    .bind(name)
    .bind(email)
    .execute(pool)
    .await?;

  // Get the last inserted row's ID (Autoincrement)
  let user_id = row.last_insert_rowid();

  let user = User {
    id: user_id,
    name: name.to_string(),
    email: email.to_string(),
  };

  Ok(user)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  // Connect to the SQLite database
  let pool = SqlitePool::connect("sqlite:study.db").await?;

  // Create a new user
  let user = create_user(&pool, "Alice", "alice@example.com").await?;
  println!("Created User: {:?}", user);

  Ok(())
}
