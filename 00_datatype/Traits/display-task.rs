use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
  pub text: String,

  #[serde(with = "ts_seconds")]
  pub created_at: DateTime<Utc>,
}
impl Task {
  pub fn new(text: String) -> Task {
    let created_at: DateTime<Utc> = Utc::now();
    Task { text, created_at }
  }
}
use std::fmt;

impl fmt::Display for Task {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
    write!(f, "{:<50} [{}]", self.text, created_at)
  }
}

fn main() {
  let task = Task::new('Hello');

  println!("{}", task);
}
