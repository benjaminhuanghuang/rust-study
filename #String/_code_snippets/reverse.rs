pub fn reverse(input: &str) -> String {
  input.chars().rev().collect()
  //input.chars().rev().collect::<String>()
  //input.graphemes(true).rev().collect::<String>()
}
