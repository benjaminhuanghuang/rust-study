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
  write_database(strKey, strValue)
}

struct Database {
  hashmap: std::collections::HashMap<String, String>,
}

impl Database {
  fn from_disk() -> Database {
    Database {
      hashmap: std::collections::HashMap::new(),
    }
  }
}

fn write_database(key: String, value: String) -> Result<(), std::io::Error> {
  let contents: String = format!("{} {}", key, value);

  std::fs::write("kv.db", contents) // return the result, no ; at the end
}
