use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  // value -> index:usize
  let mut hashmap = HashMap::new();

  for (i, num) in nums.iter().enumerate() {
    let complement = target - num;
    //check the key
    if hashmap.contains_key(&complement) {
      return vec![hashmap[&complement], i as i32];
    }
    hashmap.insert(num, i as i32);
  }
  vec![]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6));
  }
}
