use axum::{
  extract::{Path, State},
  routing::{get, post},
  Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
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

  // Define API routes
  let app = Router::new()
    .route("/users", post(create_user).get(list_users))
    .route(
      "/users/{id}",
      get(get_user).put(update_user).delete(delete_user),
    )
    .with_state(pool);

  // Set the server address
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

// User model
#[derive(Serialize, Deserialize, FromRow)]
struct User {
  id: Option<i64>,
  name: String,
  email: String,
}

// Create a new user
async fn create_user(State(pool): State<SqlitePool>, Json(payload): Json<User>) -> Json<User> {
  let inserted_user =
    sqlx::query_as::<_, User>("INSERT INTO users (id,name, email) VALUES (?,?, ?) RETURNING id")
      .bind(&payload.id)
      .bind(&payload.name)
      .bind(&payload.email)
      .fetch_one(&pool)
      .await
      .expect("Failed to insert user");

  Json(inserted_user)
}

// Get all users
async fn list_users(State(pool): State<SqlitePool>) -> Json<Vec<User>> {
  let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch users");

  Json(users)
}

// Get a user by ID
async fn get_user(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Json<User> {
  let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
    .bind(id)
    .fetch_one(&pool)
    .await
    .expect("User not found");

  Json(user)
}

// Update a user
async fn update_user(
  State(pool): State<SqlitePool>,
  Path(id): Path<i64>,
  Json(payload): Json<User>,
) -> Json<User> {
  let updated_user =
    sqlx::query_as::<_, User>("UPDATE users SET name = ? WHERE id = ? RETURNING id, name")
      .bind(&payload.name)
      .bind(id)
      .fetch_one(&pool)
      .await
      .expect("Failed to update user");

  Json(updated_user)
}

// Delete a user
async fn delete_user(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Json<String> {
  sqlx::query("DELETE FROM users WHERE id = ?")
    .bind(id)
    .execute(&pool)
    .await
    .expect("Failed to delete user");

  Json(format!("User {} deleted", id))
}
