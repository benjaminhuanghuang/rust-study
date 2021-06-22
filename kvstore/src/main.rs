use std::str::SplitN;
use core::iter::Skip;
use std::collections::HashMap;
use std::env::Args;
use std::fs;
use std::io;

fn main() {
  // let key = std::env::args().nth(1).expect("Please specify an key");

  // An iterator that skips over n elements of iter
  let mut arguments: Skip<Args> = std::env::args().skip(1);
  let key: String = arguments.next().expect("Key was not there");
  let value: String = arguments.next().unwrap();

  let contents: String = format!("{}\t{}\n", key, value);
  std::fs::write("kv.db", contents).unwrap();

  let database:Result<Database, io::Error> = Database::new();
}

struct Database {
  map: HashMap<String, String>,
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
    for line in contents.lines() {   // borrowing the string owned by contents
      let mut chunks: SplitN<char> = line.splitn(2,'\t');  // get an interator
      let key : &str = chunks.next().expect("No key");
      let value : &str = chunks.next().expect("No value");

      map.insert(key.to_owned(), value.to_owned());

    }
    // parse the file

    Ok(Database {
      map: HashMap::new(),
    })
  }
}
