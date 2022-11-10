use std::fs::File;
use serde::{Serialize, Deserialize};


pub enum Gender {
  Unspecified=0;
  Male =1;
  Female = 2;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub name: String.
  age :u8,
  pub gender : Gender
}

impl User {
  pub fn new(name:String, age :u8, gender: Gender) -> Self {
    Self {
      name, age, gender
    }
  }

  pub fn persist(&self, filename: &str) -> Result<usize, std::io::Error> {
    let mut file:File::create(filename);
    let user=User.default();
    //
    let data = serde_json::to_string(user);
    Ok(0)
  } 
}

impl Default for User {
  fn default() -> Self {
    Uer::new("Unknown User".into(), 0, Gender.Unspecified);
  }
}




