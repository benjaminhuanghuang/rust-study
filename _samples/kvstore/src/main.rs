use core::iter::Skip;
use std::collections::HashMap;
use std::env::Args;
use std::fs;
use std::io;
use std::str::SplitN;

fn main() {
  // let key = std::env::args().nth(1).expect("Please specify an key");

  // An iterator that skips over n elements of iter
  let mut arguments: Skip<Args> = std::env::args().skip(1);
  let key: String = arguments.next().expect("Key was not there");
  let value: String = arguments.next().unwrap();
  println!("The key is '{}' and the value is '{}'", key, value);

  let contents: String = format!("{}\t{}\n", key, value);
  std::fs::write("kv.db", contents).unwrap();

  let mut database = Database::new().expect("Creating db failed");
  database.insert(key.to_uppercase(), value.clone());
  database.insert(key, value);
  match database.flush() {
    Ok(()) => println!("YAY!"),
    Err(err) => println!("OH NOS! Error! {}", err),
  }
}

struct Database {
  map: HashMap<String, String>,
  flush: bool,
}

impl Database {
  // constructor
  fn new() -> Result<Database, io::Error> {
    let mut map: HashMap<String, String> = HashMap::new();
    // read the kv file
    // let c: String = match fs::read_to_string("kv.db") {
    //   Ok(c: String) => c,
    //   Err(error: Error) =>{
    //     return Result::Err(error);
    //   }
    // }
    let contents: String = fs::read_to_string("kv.db")?;
    for line in contents.lines() {
      // borrowing the string owned by contents
      let mut chunks: SplitN<char> = line.splitn(2, '\t'); // get an interator
      let key: &str = chunks.next().expect("No key");
      let value: &str = chunks.next().expect("No value");

      map.insert(key.to_owned(), value.to_owned());
    }
    // parse the file
    Ok(Database {
      map: map,
      flush: false,
    })
  }

  fn insert(&mut self, key: String, value: String) {
    self.map.insert(key, value);
  }

  fn flush(&self) -> std::io::Result<()> {
    do_flush(&self)
  }
}

impl Drop for Database {
  fn drop(&mut self) {
    if !self.flush {
      let _ = do_flush(self);
    }
  }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
  println!("Do flush called");
  let mut contents = String::new();
  for (key, value) in &database.map {
    contents.push_str(key);
    contents.push('\t');
    contents.push_str(&&&&value);
    contents.push('\n');
  }
  std::fs::write("kv.db", contents)
}
