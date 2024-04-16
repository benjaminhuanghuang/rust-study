use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
  let mut arguments = std::env::args().skip(1);

  // for arg in arguments {
  //   println!("Got arg:{}", arg);
  // }
  let key = arguments.next(); // return an Option<String>
  let value = arguments.next();
  // println!("Key: {:?}, Value: {:?}", key, value);
  // println!("Key: {}, Value: {}", key.unwrap(), value.unwrap());
  let strKey = key.unwrap();
  let strValue = value.unwrap();

  let mut db = Database::from_disk()?;
  db.insert(strKey, strValue);
  db.flush()?;

  Ok(())
}

struct Database {
  hashmap: HashMap<String, String>,
}

impl Database {
  fn from_disk() -> Result<Database, std::io::Error> {
    let contents = std::fs::read_to_string("kv.db")?;
    let mut hashmap = HashMap::new();
    for lin in contents.lines() {
      let mut key_value = lin.splitn(2, ' ');
      let key = key_value.next().expect("No key");
      let value = key_value.next().expect("No value");
      hashmap.insert(key.to_owned(), value.to_owned());
    }
    Ok(Database { hashmap })
  }

  fn insert(&mut self, key: String, value: String) {
    self.hashmap.insert(key, value);
  }

  fn flush(&self) -> Result<(), std::io::Error> {
    let contents: String = todo!("Format the keys and values in self.hashmap and return a String");
    std::fs::write("kv.db", contents)
  }
}

fn write_database(key: String, value: String) -> Result<(), std::io::Error> {
  let contents: String = format!("{} {}", key, value);

  std::fs::write("kv.db", contents) // return the result, no ; at the end
}
