use std::fmt::Debug;
/*
  Quick sorting  O(N^2)

*/

//Move first element to the correct place
//Everything lower should be before it, everything highter should be after it
//return it's location
pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
  // --- Rand p
  // let mut p = b_rand::rand(v.len());
  // v.swap(0, p);
  // p = 0;
  // --- simple version:
  let mut p = 0;
  for i in 1..v.len() {
    if v[i] < v[p] {
      //move our pivot forward 1, and put this element before it
      v.swap(p + 1, i);
      v.swap(p, p + 1);
      p += 1
    }
  }
  p
}

pub fn quick_sort<T: Send + PartialOrd + Debug>(v: &mut [T]) {
  if v.len() <= 1 {
    return;
  }
  let p = pivot(v);
  println!("Quick sort {:?}", v);

  let (a, b) = v.split_at_mut(p);

  quick_sort(a);
  quick_sort(&mut b[1..]); // middle element already sorted
}

struct RawSend<T>(*mut [T]); //one element tuple

unsafe impl<T> Send for RawSend<T> {}

pub fn threaded_quick_sort<T: 'static + PartialOrd + Debug + Send>(v: &mut [T]) {
  if v.len() <= 1 {
    return;
  }
  let p = pivot(v);
  println!("{:?}", v);

  let (a, b) = v.split_at_mut(p);

  let raw_a: *mut [T] = a as *mut [T];
  let raw_s = RawSend(raw_a);

  unsafe {
    let handle = std::thread::spawn(move || {
      threaded_quick_sort(&mut *raw_s.0);
    });

    threaded_quick_sort(&mut b[1..]);

    //compiler doesn't know that we join these
    //We do
    handle.join().ok();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pivot() {
    let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];
    let p = pivot(&mut v);

    for x in 0..v.len() {
      assert!((v[x] < v[p]) == (x < p));
    }
    //assert_eq!(v, vec![1, 3, 4, 6, 19, 8, 11, 13]);
  }

  #[test]
  fn test_quick_sort() {
    let mut v = vec![4, 6, 1, 8, 11, 13, 3];
    quick_sort(&mut v);
    assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);

    let mut v = vec![1, 2, 6, 7, 9, 12, 13, 14];
    quick_sort(&mut v);
    assert_eq!(v, vec![1, 2, 6, 7, 9, 12, 13, 14]);
  }
}
