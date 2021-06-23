use std::time::{Duration, Instant};


pub fn run() {
  let time_limit = Duration::new(1, 0);
  let start = Instant::now();

  let mut count = 0;
  while (Instant::now() - start < time_limit) {
    count += 1;
  }

  println!("{}", count);
}
