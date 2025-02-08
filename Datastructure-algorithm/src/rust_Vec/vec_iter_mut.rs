let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();


fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
  for i in v.iter_mut() {
    // TODO: Fill this up so that each element in the Vec `v` is
    // multiplied by 2.
    //*i = *i * 2;
    *i *= 2;
  }
  // At this point, `v` should be equal to [4, 8, 12, 16, 20].
  v
}

// immutable iter
fn vec_map(v: &Vec<i32>) -> Vec<i32> {
  v.iter().map(|num| {
      // TODO: Do the same thing as above - but instead of mutating the
      // Vec, you can just return the new number!
      num * 2
  }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
