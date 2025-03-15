fn vec_map(input: &[i32]) -> Vec<i32> {
  input.iter().map(|element| element * 2).collect()
}

fn test_vec_map() {
  let input = [2, 4, 6, 8, 10];
  let ans = vec_map(&input);
  assert_eq!(ans, [4, 8, 12, 16, 20]);
}
