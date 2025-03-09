use sqlx::{sqlite::SqlitePool, Error, FromRow};

#[tokio::main]
async fn main() -> Result<(), Error> {
  // Connect to the SQLite database
  let pool = SqlitePool::connect("sqlite:study.db").await?;

  // Run migrations (if needed)
  sqlx::migrate!().run(&pool).await?;

  // Create a new user
  let user = create_user(&pool, "Alice", "alice@example.com").await?;
  println!("Created User: {:?}", user);

  // Read all users
  let users = list_users(&pool).await?;
  println!("All Users: {:?}", users);

  // Get a single user by ID
  let user_id = &user.id;
  let single_user = get_user(&pool, user_id).await?;
  println!("Single User: {:?}", single_user);

  // Update a user
  let updated_user = update_user(&pool, user_id, "Alice Updated", "alice.new@example.com").await?;
  println!("Updated User: {:?}", updated_user);

  // Delete a user
  delete_user(&pool, user_id).await?;
  println!("Deleted User with ID: {}", user_id);

  Ok(())
}

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

// Read all users
async fn list_users(pool: &SqlitePool) -> Result<Vec<User>, Error> {
  let users = sqlx::query_as::<_, User>("SELECT * FROM users")
    .fetch_all(pool)
    .await?;

  Ok(users)
}

// Read a single user by ID
async fn get_user(pool: &SqlitePool, user_id: &i64) -> Result<User, Error> {
  let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
    .bind(user_id)
    .fetch_one(pool)
    .await?;

  Ok(user)
}

// Update user
async fn update_user(
  pool: &SqlitePool,
  user_id: &i64,
  name: &str,
  email: &str,
) -> Result<User, Error> {
  let query = "UPDATE users SET name = ?, email = ? WHERE id = ?";
  sqlx::query(query)
    .bind(name)
    .bind(email)
    .bind(user_id)
    .execute(pool)
    .await?;

  // Return updated user
  Ok(User {
    id: *user_id,
    name: name.to_string(),
    email: email.to_string(),
  })
}

// Delete user
async fn delete_user(pool: &SqlitePool, user_id: &i64) -> Result<(), Error> {
  let query = "DELETE FROM users WHERE id = ?";
  sqlx::query(query).bind(user_id).execute(pool).await?;

  Ok(())
}
