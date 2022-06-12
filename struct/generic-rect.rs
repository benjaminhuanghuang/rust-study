struct Rectangle<T, U> {
  width: T,
  height: U,
}

impl<T, U> Rectangle<T, U> {
  fn get_width(&self) -> &T {
    // use & because we don't know the type of T
    &self.width
  }

  fn scale(&mut self, scalar: f64) {
    self.width *= scalar;
    self.height *= scalar;
  }

  fn new(width: f64, height: f64) -> Rectangle {
    Rectangle { width, height }
  }
}

impl Rectangle<u8, u8> {
  fn get_permieter(&self) -> &u8 {
    // use & because we don't know the type of T
    2 * (self.width + self.height)
  }
}

fn main() {
  let mut rect = Rectangle::new(1.2, 3.4);
  assert_eq!(rect.get_area(), 4.08);

  rect.scale(0.5);
  assert_eq!(rect.get_area(), 1.02);

  println!("Tests passed!");
}
