#[derive(Debug)]
pub struct Person {
  name: String,
  age: u32,
  children: i32,
}

impl Person {
  pub fn print(self) -> String {
    format!(
      "name={}, age={} has {} children",
      self.name, self.age, self.children
    )
  }
}

fn main() {
  let p = Person {
    name: "ben".to_string(),
    age: 50,
    children: 1,
  };

  //println!("Hello, from {}", p.print());
  println!("Hello, from debug {:?}", p);
}
