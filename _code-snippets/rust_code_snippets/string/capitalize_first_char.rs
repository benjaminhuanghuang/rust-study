// The trait `AppendBar` has only one function which appends "Bar" to any object
// implementing this trait.
trait AppendBar {
  fn append_bar(self) -> Self;
}

impl AppendBar for String {
  // TODO: Implement `AppendBar` for the type `String`.
  fn append_bar(mut self) -> Self {
    let bar = "bar";
    let capitalized = bar[0..1].to_uppercase() + &bar[1..]; // Uppercase the first character of "bar" and keep the rest
    self.push_str(&capitalized); // Append "bar" to the string.
    self // Return the modified string.
  }
}

fn main() {
  let s = String::from("Foo");
  let s = s.append_bar();
  println!("s: {s}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_foo_bar() {
    assert_eq!(String::from("Foo").append_bar(), "FooBar");
  }

  #[test]
  fn is_bar_bar() {
    assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
  }
}
