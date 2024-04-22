fn set_char(text: &str, col: usize, c: char) -> String {
  let mut chars: Vec<char> = text.chars().collect();
  chars[col] = c;
  chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut s = String::from("hello");
    let s = set_char(&s, 2, 'x');

    assert_eq!(s, "hexlo");
  }
}
