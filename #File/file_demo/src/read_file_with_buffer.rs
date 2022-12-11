use std::fs::File;
use std::io::{self, BufRead};

fn main() {
  let file_path = "test.txt"
  
  let file: File = File::open(file_path).unwrap();
  
  let reader = BufReader::new(file);

  for line in reader.lines() {
    println!("{}", line.unwrap());
  }
}
