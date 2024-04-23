pub struct RandGen {
  curr: usize,
  mul: usize,
  inc: usize,
  modulo: usize,
}

impl RandGen {
  pub fn new(curr: usize) -> Self {
    Self {
      curr,
      mul: 56394237,
      inc: 346423491,
      modulo: 23254544563,
    }
  }

  pub fn next_v(&mut self, max: usize) -> usize {
    self.curr = (self.curr * self.mul + self.inc) % self.modulo;
    self.curr % max
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rands_pringout() {
    let mut r = RandGen::new(12);
    for _ in 0..100 {
      println!("--{}", r.next_v(100));
    }
  }
}
