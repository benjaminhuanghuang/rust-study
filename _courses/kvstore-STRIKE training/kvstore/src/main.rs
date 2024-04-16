fn main() {
  let mut arguments = std::env::args().skip(1);

  // for arg in arguments {
  //   println!("Got arg:{}", arg);
  // }
  let key = arguments.next(); // return an Option<String>
  let value = arguments.next();
  // println!("Key: {:?}, Value: {:?}", key, value);
  // println!("Key: {}, Value: {}", key.unwrap(), value.unwrap());

  let write_result = std::fs::write("kv.db", "Hello world!");
  match write_result {
    Ok(()) => {
      println!("Write successful");
    }
    Err(e) => {
      println!("Write failed: {}", e);
    }
  }
}
