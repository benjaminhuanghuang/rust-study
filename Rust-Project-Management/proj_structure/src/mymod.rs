use super::add::add_one::add_one;

fn test() {
  print!("{}", add_one(0));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    assert_eq!(crate::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
  }
}
