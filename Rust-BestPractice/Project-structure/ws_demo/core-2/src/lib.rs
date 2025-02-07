pub fn print_from_core2() {
  println!("print from core-2");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {}
}
