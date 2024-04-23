use std::fmt::Debug;

// Time complexity O(N*log(N))
// Space complexity O(N)
pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
  // sort the left half
  // sort the right half O(n + 2 * n/2 + 4 * n/4 + ... + n*log(n)/n) = O(N*log(N)
  // bring the sorted halves together O(N)

  if v.len() <= 1 {
    return v;
  }

  let mut res = Vec::with_capacity(v.len());

  let b = v.split_off(v.len() / 2); //mut v
  let a = merge_sort(v);
  let b = merge_sort(b); //shadow the previous b

  let mut a_it = a.into_iter();
  let mut b_it = b.into_iter();
  let mut a_peek = a_it.next();
  let mut b_peek = b_it.next();

  loop {
    match a_peek {
      Some(ref a_val) => match b_peek {
        Some(ref b_val) => {
          if a_val < b_val {
            res.push(a_peek.take().unwrap());
            a_peek = a_it.next();
          } else {
            res.push(b_peek.take().unwrap());
            b_peek = b_it.next();
          }
        }
        None => {
          res.push(a_peek.take().unwrap());
          res.extend(a_it); // append the rest of the elements
          return res;
        }
      },
      None => {
        if let Some(b_val) = b_peek {
          res.push(b_val);
        }
        res.extend(b_it);
        return res;
      }
    }
  }
}

#[cfg(test)] // don't include normal code
mod tests {
  use super::*;

  #[test]
  fn test_merge_sort() {
    let v = vec![4, 6, 1, 8, 11, 13, 3];
    let v = merge_sort(v);
    assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
  }
}
