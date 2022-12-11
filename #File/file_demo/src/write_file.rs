use std::fs::*;

fn main() {
  let file_path = "test.txt"
  let content = "abcd".to_string();

  fs::write(file_path, content).unwrap();
}
