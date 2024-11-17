pub fn create_message_name(name: &str) -> String {
  if name.trim().is_empty() {
    panic!("Name can not be blank")
  }

  format!("Hello, {}!", name)
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  #[should_panic(expected = "Name cannot be blank")]
  fn test_blank_name() {
    create_message_name("name");
  }
}
