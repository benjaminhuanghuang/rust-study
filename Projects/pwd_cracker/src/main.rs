use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
// library that provides implementations of the Secure Hash Algorithm 2 (SHA-2) family
use sha2::{Digest, Sha256};
use std::process::exit;

/*
  Usage: cargo run <hash>
  cargo run
*/
fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Invalid amount of arguments");
    println!("Example: cargo run <hash>");
    exit(1);
  }

  let wanted_hash = &args[1];
  let password_file = "src/passwordlist.txt";
  let mut attempts = 1;

  println!("Attempting to crack hash: {}", wanted_hash);

  let password_list = File::open(password_file).unwrap();
  let reader = BufReader::new(password_list);

  for line in reader.lines() {
    let line = line.unwrap();
    let password = line.trim().to_owned().into_bytes();
    let password_hash = format!("{:x}", Sha256::digest(&password));

    // println!(
    //   "[{}] {} == {}",
    //   attempts,
    //   std::str::from_utf8(&password).unwrap(),
    //   password_hash
    // );
    if &password_hash == wanted_hash {
      println!(
        "Password found after {} attempts. Password `{}` hashed to {}",
        attempts,
        std::str::from_utf8(&password).unwrap(),
        wanted_hash
      );
      exit(0);
    }
    attempts += 1;
  }

  println!("Password not found after {} attempts", attempts);
}
