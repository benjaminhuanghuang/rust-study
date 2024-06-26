use std::fmt::Debug;

pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
  for _ in 0..v.len() - 1 {
    for i in 0..v.len() - 1 {
      if v[i] > v[i + 1] {
        v.swap(i, i + 1)
      }
    }
  }
}

pub fn bubble_sort_2<T: PartialOrd>(v: &mut [T]) {
  for _ in 0..v.len() - 1 {
    let mut sorted = true;
    for i in 0..v.len() - 1 {
      if v[i] > v[i + 1] {
        v.swap(i, i + 1);
        sorted = false;
      }
    }
    if sorted {
      return;
    }
  }
}

// O(N*N)
pub fn bubble_sort_3<T: PartialOrd + Debug>(v: &mut [T]) {
  for p in 0..v.len() {
    print!("{:?}", v);
    let mut sorted = true;
    for i in 0..v.len() - 1 - p {
      if v[i] > v[i + 1] {
        v.swap(i, i + 1);
        sorted = false;
      }
    }
    if sorted {
      return;
    }
  }
}

#[cfg(test)] // don't include normal code
mod tests {
  use super::*;

  #[test]
  fn test_bubble_sort() {
    let mut v = vec![4, 6, 1, 8, 11, 13, 3];
    bubble_sort(&mut v);
    assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
  }

  #[test]
  fn test_bubble_sort_2() {
    let mut v = vec![4, 6, 1, 8, 11, 13, 3];
    bubble_sort_2(&mut v);
    assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
  }

  #[test]
  fn test_bubble_sort_3() {
    let mut v = vec![4, 6, 1, 8, 11, 13, 3];
    bubble_sort_3(&mut v);
    assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
  }
}
