fn main() {
  let mut word = String::new();
  while word.trim() != "rust" {
    println!("What's the secret word?");
    io::stdin()
      .read_line(&mut word)
      .expect("Failed to read line");
  }

  println!("You know th secret word!");
}
