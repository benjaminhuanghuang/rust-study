use dotenvy::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct User {
  name: String,
  age: u32,
  email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // Load .env file
  dotenv().ok();
  let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI not found in .env file");
  // Create client
  let client = Client::with_uri_str(&mongo_uri).await?;

  // Select database
  let db = client.database("mydatabase");
  let collection: Collection<User> = db.collection("users");

  // Insert a user
  let new_user = User {
    name: "Alice".to_string(),
    age: 25,
    email: "alice@example.com".to_string(),
  };
  collection.insert_one(new_user).await?;
  println!("✅ User inserted");

  // Find a user
  let filter = doc! { "name": "Alice" };
  if let Some(user) = collection.find_one(filter.clone()).await? {
    println!("🔍 Found user: {:?}", user);
  }

  // Update a user
  let update = doc! { "$set": { "age": 26 } };
  collection.update_one(filter.clone(), update).await?;
  println!("✅ User updated");

  // Delete a user
  // collection.delete_one(filter).await?;
  println!("🗑️ User deleted");
  println!("✅ Successfully connected to MongoDB Atlas!");
  Ok(())
}
