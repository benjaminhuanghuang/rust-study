use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct User {
  id: u32,
  name: String,
  email: String,
}

impl User {
  fn load(file_path: &str) -> Result<Self, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let user: User = serde_json::from_str(&contents)?;
    Ok(user)
  }
}

fn main() {
  let file_path = "user.json"; // Ensure this file exists
  match User::load(file_path) {
    Ok(user) => println!("Loaded user: {:?}", user),
    Err(e) => eprintln!("Error loading user: {}", e),
  }
}
