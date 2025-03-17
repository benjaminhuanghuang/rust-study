fn array_and_vec() -> ([i32; 4], Vec<i32>) {
  let a = [10, 20, 30, 40]; // Array

  // Use the vector macro.
  // let v = ???;
  let v = vec![a];
  let v = Vec::from(a);
  let v = a.to_vec();

  (a, v)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_array_and_vec() {
    let (a, v) = array_and_vec();
    assert_eq!(a, [10, 20, 30, 40]);
    assert_eq!(v, vec![10, 20, 30, 40]);
  }

  #[test]
  fn test_array_length() {
    let (a, _) = array_and_vec();
    assert_eq!(a.len(), 4);
  }

  #[test]
  fn test_vec_length() {
    let (_, v) = array_and_vec();
    assert_eq!(v.len(), 4);
  }

  #[test]
  fn test_array_elements() {
    let (a, _) = array_and_vec();
    assert_eq!(a[0], 10);
    assert_eq!(a[1], 20);
    assert_eq!(a[2], 30);
    assert_eq!(a[3], 40);
  }

  #[test]
  fn test_vec_elements() {
    let (_, v) = array_and_vec();
    assert_eq!(v[0], 10);
    assert_eq!(v[1], 20);
    assert_eq!(v[2], 30);
    assert_eq!(v[3], 40);
  }
}
