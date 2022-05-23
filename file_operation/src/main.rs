use std::fs::*;

fn main() {
  // Create file
  let f = File::create("./temp.txt");

  // Open file
  let f1 = OpenOptions::new()
    .write(true)
    .read(true)
    .create(true)
    .open("./test.txt");

  println!("f: {:?}, f1: {:?}", f, f1);

  let open_file = File::open("./notexist.txt");
}
