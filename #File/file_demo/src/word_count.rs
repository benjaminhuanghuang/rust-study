use collections::HashMap;
use std::fs::*;

fn get_words(text: &str) -> Vec<String> {
  text
    .split_whitespace()
    .map(|word| word.to_string())
    .collect()
}

fn word_count(words: &Vec<String>) -> HashMap<String, i32> {
  let mut hashmap = HashMap::new();
  for word in words {
    hashmap
      .entry(word.to_lowercase())
      .and_modify(|count| *count += 1)
      .or_insert(1);
  }
  hashmap
}

fn main() {
  let contents = String::from("This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five");
  let words = get_words(&contents);
  let counter = word_count(&words);

  println!("{:#?}", counter);
}
