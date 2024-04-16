use std::ptr::NonNull;
use std::alloc;

pub struct MyVec<T> {
  ptr: NonNull<T>,
  len: usize,
  capacity: usize,
}

impl<T> MyVec<T> {
  pub fn new() -> Self {
    Self {
      ptr: NonNull::dangling(),
      len: 0,
      capacity: 0,
    }
  }

 pub push(&mut self, value: T) {

 }

  pub fn capacity(&self) -> usize {
    self.capacity
  }

  pub fn len(&self) -> usize {
    self.len
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut vec: MyVec<i32> = MyVec::new();
    // vec.push(1usize);
    // vec.push(1usize);
    // vec.push(1usize);

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.len(), 0);
  }
}
