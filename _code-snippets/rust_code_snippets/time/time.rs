use std::time::{SystemTime, UNIX_EPOCH};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
  start + Duration::from_secs(11111)
}

fn main() {
  match SystemTime::now().duration_since(UNIX_EPOCH) {
    Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
    Err(_) => panic!("SystemTime before UNIX EPOCH!"),
  }
}
