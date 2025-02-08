pub struct RingBuffer<T> {
  storage: Vec<T>,
  head: usize,
  tail: usize,
  size: usize,
}
/*
The first T is used to define the type parameter for the implementation block.
The second T refers to the type parameter for the RingBuffer struct itself
*/
impl<T> RingBuffer<T> {
  // Create a new ring storage with a given capacity
  pub fn new(capacity: usize) -> Self {
    RingBuffer {
      storage: Vec::with_capacity(capacity),
      head: 0,
      tail: 0,
      size: 0,
    }
  }

  // Add an element to the storage
  pub fn push(&mut self, item: T) {
    if self.size == self.storage.capacity() {
      panic!("Buffer is full");
    }
    if self.size == self.storage.len() {
      self.storage.push(item);
    } else {
      self.storage[self.tail] = item;
    }
    self.tail = (self.tail + 1) % self.storage.capacity();
    self.size += 1;
  }

  // Remove and return the oldest element from the storage
  pub fn pop(&mut self) -> Option<T> {
    if self.size == 0 {
      return None;
    }
    let item = std::mem::replace(&mut self.storage[self.head], unsafe { std::mem::zeroed() });
    self.head = (self.head + 1) % self.storage.capacity();
    self.size -= 1;
    Some(item)
  }

  // Check if the storage is empty
  pub fn is_empty(&self) -> bool {
    self.size == 0
  }

  // Check if the storage is full
  pub fn is_full(&self) -> bool {
    self.size == self.storage.capacity()
  }

  // Get the current size of the storage
  pub fn size(&self) -> usize {
    self.size
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ring_buffer() {
    let mut storage = RingBuffer::<i32>::new(3);
    assert!(storage.is_empty());
    assert!(!storage.is_full());

    storage.push(1);
    storage.push(2);
    storage.push(3);

    assert_eq!(storage.size(), 3);
    assert!(storage.is_full());

    assert_eq!(storage.pop(), Some(1));
    assert_eq!(storage.pop(), Some(2));
    assert_eq!(storage.pop(), Some(3));
    assert!(storage.is_empty());
  }
}
