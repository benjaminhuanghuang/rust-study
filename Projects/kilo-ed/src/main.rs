use crossterm::{
  event::{read, Event, KeyCode, KeyEvent},
  terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  terminal::enable_raw_mode()?;

  loop {
    if let Ok(event) = read() {
      if let Event::Key(key_event) = event {
        if key_event.code == KeyCode::Char('q') {
          break;
        } else {
          print!("{:?}\r", key_event);
        }
      } else {
        break;
      }
    }
  }
  terminal::disable_raw_mode()?;
  Ok(())
}
