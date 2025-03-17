pub fn main() {
  // Example of formatting a string
  let name = "Alice";
  let age = 30;

  // Using format! macro to create a formatted string
  let formatted_string = format!("My name is {} and I am {} years old.", name, age);

  // Print the formatted string
  println!("{}", formatted_string);
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let name = "Alice";
    let age = 30;
    let formatted_string = format!("My name is {} and I am {} years old.", name, age);
    assert_eq!(formatted_string, "My name is Alice and I am 30 years old.");
  }
}
