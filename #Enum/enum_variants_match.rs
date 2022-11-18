struct Point {
  x: u8,
  y: u8,
}

enum Message {
  Move(Point),
  Echo(String),
  ChangeColor((u8, u8, u8)),
  Quit,
}

impl State {
  fn change_color(&mut self, color: (u8, u8, u8)) {
    self.color = color;
  }

  fn quit(&mut self) {
    self.quit = true;
  }

  fn echo(&self, s: String) {
    println!("{}", s);
  }

  fn move_position(&mut self, p: Point) {
    self.position = p;
  }

  fn process(&mut self, message: Message) {
    match message {
      Message::Move(point) => self.move_position(point),
      Message::ChangeColor(color) => self.change_color(color),
      Message::Echo(message) => self.echo(message),
      Message::Quit => self.quit(),
    }
  }
}
