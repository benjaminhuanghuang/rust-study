fn is_all_digits(s: &str) -> bool {
  for char in s.chars() {
    if !char.is_digit(10) {
      return false;
    }
  }
  true
}

fn is_all_digits2(s: &str) -> bool {
  s.chars().all(|c| c.is_digit(10))
}
