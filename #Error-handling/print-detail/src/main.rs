use std::fs::OpenOptions;
use std::io::Result;

fn main() {
  let r = process_file("/dddd");
  match r {
    Ok(()) => {}
    Err(e) => {
      println!("error int process_file is  '{}' ", e);
    }
  }
}

fn process_file(path: &str) -> Result<()> {
  let file = OpenOptions::new()
    .write(true)
    .read(true)
    .create(false)
    .open(path)
    .map_err(|e| {
      println!("Failed to open path '{}', {}", path, e);
      e
    })?;
  println!("Error: {:#?}", file);
  Ok(())
}
