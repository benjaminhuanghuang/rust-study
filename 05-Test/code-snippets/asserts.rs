pub fn bigger(a: i32, b: i32) -> i32 {
  if a > b {
    a
  } else {
    b
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /*-------------------------------------------------
    assert!()
  -------------------------------------------------*/
  #[test]
  fn you_can_assert() {
    assert!(true);
  }

  /*-------------------------------------------------
    assert_eq!()
  -------------------------------------------------*/
  #[test]
  fn you_can_assert_eq() {
    assert_eq!(3, 3);
  }

  #[test]
  fn ten_is_bigger_than_eight() {
    assert_eq!(10, bigger(10, 8));
  }

  #[test]
  fn fortytwo_is_bigger_than_thirtytwo() {
    assert_eq!(42, bigger(32, 42));
  }
}
