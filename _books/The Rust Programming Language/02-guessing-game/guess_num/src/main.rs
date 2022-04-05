use rand::prelude::*;
use std::io;

fn main() {
  let secret_num = rand::thread_rng().gen_range(1..100);
  println!("Guess the number!");

  println!("Please input your guess.");

  loop {
    let mut buffer = String::new();

    let guess = match io::stdin().read_line(&mut buffer) {
      Ok(_) => match buffer.trim().parse::<u32>() {
        Ok(value) => value, //
        Err(_) => {
          println!("\n Failed to parse input. Guess again:");
          continue;
        }
      },
      Err(_) => {
        println!("\n Failed to read input. Guess again:");
        continue;
      }
    };

    if guess == secret_num {
      println!("\n You got it. the number is {}", secret_num);
      break;
    }
  }
}
