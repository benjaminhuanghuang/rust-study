fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  // value -> index:usize
  let mut hash = HashMap::new();

  for (i, num) in nums.iter().enumerate() {
    let complement = target - num;
    //check the key
    if hash.contains_key(&complement) {
      return vec![hash[&complement], i as i32];
    }
    hash.insert(num, i as i32);
  }
  vec![]
}
