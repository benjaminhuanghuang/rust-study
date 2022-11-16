fn summation(n: i32) -> i32 {
  (1..n + 1).fold(0, |acc, v| acc + v)
}

fn wise_summation(n: i32) -> i32 {
  (1..=n).sum()
}
