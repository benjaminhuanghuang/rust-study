use std::env;
fn main() {
  for argument in env::args() {
    println!("{}", argument);
  }

  let args: Vec<String> = env::args().collect();
  let size = &args[1];
  let mode = &args[2];
  let source_folder = &args[3];
  println!(
    "Size:{},mode:{},source folder: {}",
    size, mode, source_folder
  );
}
