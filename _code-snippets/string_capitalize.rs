fn capitalize_first(input: &str) -> String {
  let mut chars = input.chars();
  match chars.next() {
    None => String::new(),
    Some(first) => {
      let capitalized = first.to_uppercase().to_string();
      let rest: String = chars.collect();
      format!("{}{}", capitalized, rest)
    }
  }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
  // ???
  words.iter().map(|&word| capitalize_first(word)).collect()
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
  // ???
  words
    .iter()
    .map(|&word| capitalize_first(word))
    .collect::<Vec<String>>()
    .join("")
}
