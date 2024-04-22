/*
Heap sort:
   1. Convert the input array to a max heap.
   2. Partition the array into heap part and sorted part. Initially the
      heap consists of the whole array and the sorted part is empty:
      ```text
      arr: [ heap                    |]
      ```

      Repeatedly swap the root (i.e. the largest) element of the heap with
      the last element of the heap and increase the sorted part by one:
      ```text
      arr: [ root ...   last | sorted ]
       --> [ last ... | root   sorted ]
      ```

      After each swap, fix the heap to make it a valid max heap again.
      Once the heap is empty, `arr` is completely sorted.
*/
pub fn heap_sort<T: Ord>(arr: &mut [T]) {
  if arr.len() <= 1 {
    return; // already sorted
  }

  heapify(arr);

  for end in (1..arr.len()).rev() {
    arr.swap(0, end);
    move_down(&mut arr[..end], 0);
  }
}

/// Convert `arr` into a max heap.
fn heapify<T: Ord>(arr: &mut [T]) {
  let last_parent = (arr.len() - 2) / 2;
  for i in (0..=last_parent).rev() {
    move_down(arr, i);
  }
}

/// Move the element at `root` down until `arr` is a max heap again.
///
/// This assumes that the subtrees under `root` are valid max heaps already.
fn move_down<T: Ord>(arr: &mut [T], mut root: usize) {
  let last = arr.len() - 1;
  loop {
    let left = 2 * root + 1;
    if left > last {
      break;
    }
    let right = left + 1;
    let max = if right <= last && arr[right] > arr[left] {
      right
    } else {
      left
    };

    if arr[max] > arr[root] {
      arr.swap(root, max);
    }
    root = max;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sorting::have_same_elements;
  use crate::sorting::is_sorted;

  #[test]
  fn empty() {
    let mut arr: Vec<i32> = Vec::new();
    let cloned = arr.clone();
    heap_sort(&mut arr);
    assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
  }

  #[test]
  fn single_element() {
    let mut arr = vec![1];
    let cloned = arr.clone();
    heap_sort(&mut arr);
    assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
  }

  #[test]
  fn sorted_array() {
    let mut arr = vec![1, 2, 3, 4];
    let cloned = arr.clone();
    heap_sort(&mut arr);
    assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
  }

  #[test]
  fn unsorted_array() {
    let mut arr = vec![3, 4, 2, 1];
    let cloned = arr.clone();
    heap_sort(&mut arr);
    assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
  }

  #[test]
  fn odd_number_of_elements() {
    let mut arr = vec![3, 4, 2, 1, 7];
    let cloned = arr.clone();
    heap_sort(&mut arr);
    assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
  }

  #[test]
  fn repeated_elements() {
    let mut arr = vec![542, 542, 542, 542];
    let cloned = arr.clone();
    heap_sort(&mut arr);
    assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
  }
}
