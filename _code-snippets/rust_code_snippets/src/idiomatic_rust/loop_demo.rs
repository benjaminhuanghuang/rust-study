#[cfg(test)]
mod tests {
  #[test]
  fn loop_demo() {
    let mut n = 0;
    loop {
      if n > 0 {
        println!("n is now {}", n);
        n -= 1;
      }
    }
  }

  #[test]
  fn for_demo() {
    for i in 0..10 {
      print!("{} ", i);
    }
  }
}
