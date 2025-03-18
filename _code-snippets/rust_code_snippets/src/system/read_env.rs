use dotenv::dotenv;
use std::env;

fn main() {
  dotenv().ok();

  // returns an iterator of key-value pairs for all environment variables of the current process.
  for (key, value) in env::vars() {
    println!("{}:{}", key, value);
  }

  println!("Value of size is {}", env::var("size").unwrap());
}
