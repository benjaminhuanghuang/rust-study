```rs
#[derive(Debug, Serialize, Deserialize)]
pub struct Difficulty {
  level: u32,
}

impl Display for Difficulty {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    match self.level {
      1 => f.write_str("Easy"),
      2 => f.write_str("Medium"),
      3 => f.write_str("Hard"),
      _ => f.write_str("Unknown"),
    }
  }
}


// Usage
difficulty.to_string()
```
