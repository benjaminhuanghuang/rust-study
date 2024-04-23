use std::fmt::Debug;

// Move the first element to the correct position
// smaller elements to the left, larger elements to the right
pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
  let mut p = 0;
  for i in 1..v.len() {
    if v[i] < v[p] {
      v.swap(p + 1, i);
      v.swap(p, p + 1);
      p += 1;
    }
  }
  p
}

// O(N*log(N))
pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
  if v.len() <= 1 {
    return;
  }
  let p = pivot(v);
  println!("{:?}", v);
  let (a, b) = v.split_at_mut(p);
  quick_sort(a);
  quick_sort(&mut b[1..]); // Middle element already sorted
}

#[cfg(test)] // don't include normal code
mod tests {
  use super::*;

  #[test]
  fn test_pivot() {
    let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];
    let p = pivot(&mut v);
    for x in 0..v.len() {
      assert!((v[x] < v[p]) == (x < p));
    }
    assert_eq!(v, vec![1, 3, 4, 6, 19, 8, 11, 13]); // make sure 4 is in the correct position
  }

  #[test]
  fn test_quick_sort() {
    let mut v = vec![4, 6, 1, 8, 11, 13, 3];
    quick_sort(&mut v);
    assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
  }
}
