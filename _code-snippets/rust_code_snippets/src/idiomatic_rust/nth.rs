pub fn nth(n: u32) -> u32 {
  (2u32..)
    // filter x not divisible by any number from 2 to square root of x.
    .filter(|&x| (2..=(x as f32).sqrt() as u32).all(|y| x % y != 0))
    .nth(n as usize)
    .unwrap()
}
