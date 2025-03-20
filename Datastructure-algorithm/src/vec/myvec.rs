/*

Implementing Rust's Vec From Scratch
https://www.youtube.com/watch?v=3OL95gZgPWA
*/
use std::ptr::NonNull;
pub struct MyVec<T> {
  /*
    raw pointer (NonNull<T>) to the heap-allocated memory that stores the elements of the vector.
    Using raw pointers allows for manual memory management
  */
  ptr: NonNull<T>,
  len: usize,
  capacity: usize,
}

impl<T> MyVec<T> {
  pub fn new() -> Self {
    MyVec {
      ptr: NonNull::dangling(),
      len: 0,
      capacity: 0,
    }
  }

  pub fn push(&mut self, value: T) {
    if self.len == self.capacity {
      self.resize();
    }
    unsafe {
      let ptr = self.ptr.as_ptr().add(self.len);
      std::ptr::write(ptr, value);
    }
    self.len += 1;
  }

  fn resize(&mut self) {
    let new_capacity = if self.capacity == 0 {
      1
    } else {
      self.capacity * 2
    };
    let new_ptr = unsafe {
      let layout = std::alloc::Layout::array::<T>(new_capacity).unwrap();
      std::alloc::realloc(
        self.ptr.as_ptr() as *mut u8,
        layout,
        new_capacity * std::mem::size_of::<T>(),
      )
    };
    self.ptr = NonNull::new(new_ptr as *mut T).unwrap();
    self.capacity = new_capacity;
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn capacity(&self) -> usize {
    self.capacity
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let mut vec: Vec<usize> = Vec::new();
    vec.push(1usize);
    vec.push(1usize);
    vec.push(1usize);
    assert_eq!(vec.capacity(), 4);
    assert_eq!(vec.len(), 3)
  }
}
