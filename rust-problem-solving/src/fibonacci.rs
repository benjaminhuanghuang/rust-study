fn fibonacci(n: u32) -> u32 {
  let mut a = 0; // F(0)
  let mut b = 1; // F(1)

  for _ in 0..n {
    let temp = a;
    a = b;
    b = temp + b; // next Fibonacci
  }

  a
}

fn fib(n: u32) -> u32 {
  if n < 2 {
    return n;
  } else {
    return fib(n - 1) + fib(n - 2);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fibonacci() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(6), 8);
    assert_eq!(fibonacci(7), 13);
    assert_eq!(fibonacci(8), 21);
    assert_eq!(fibonacci(9), 34);
    assert_eq!(fibonacci(10), 55);
  }
}
