#[derive(Debug)]
pub enum Color {
  Red(String),
  Green,
}

fn main() {
  let c = Color::Red("255,0,0".to_string());

  match c {
    Color::Red(s) => println!("It is red {}", s),
    Color::Green => println!("It is Green"),
  }
}
