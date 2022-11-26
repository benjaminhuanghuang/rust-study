trait AppendBar {
  fn append_bar(self) -> Self;
}

impl AppendBar for String {
  //Add your code here
  fn append_bar(mut self) -> Self {
    self.push_str("Bar");
    self
  }
}

impl AppendBar for Vec<String> {
  fn append_bar(mut self) -> Self {
    self.push(String::from("Bar"));
    self
  }
}

fn main() {
  let s = String::from("Foo");
  let s = s.append_bar();
  println!("s: {}", s);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_foo_bar() {
    assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
  }

  #[test]
  fn is_bar_bar() {
    assert_eq!(
      String::from("").append_bar().append_bar(),
      String::from("BarBar")
    );
  }

  #[test]
  fn is_vec_pop_eq_bar() {
    let mut foo = vec![String::from("Foo")].append_bar();
    assert_eq!(foo.pop().unwrap(), String::from("Bar"));
    assert_eq!(foo.pop().unwrap(), String::from("Foo"));
  }
}
