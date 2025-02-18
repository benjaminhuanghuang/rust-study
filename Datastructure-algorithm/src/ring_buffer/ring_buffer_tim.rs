/*
Writing a Rust-based ring buffer By Tim McNamara

https://en.wikipedia.org/wiki/Circular_buffer

https://www.youtube.com/watch?v=TQVwv_e_rMw

https://gist.github.com/timClicks/68955827abfe0ff964ce1a61996afed9

*/
#[derive(Debug)]
pub struct RingBuffer<T: Clone> {
  storage: Vec<Option<T>>,
  read_idx: usize,
  write_idx: usize,
}
#[derive(Debug, PartialEq)]
struct Full;

/*
The first T is used to define the type parameter for the implementation block.
The second T refers to the type parameter for the RingBuffer struct itself

If T does not implement Clone, we cannot easily retrieve elements:

fn get(&self, index: usize) -> Option<T> {
    self.storage[index] // Error: Cannot move out of `Option<T>`
}
*/
impl<T: Clone> RingBuffer<T> {
  pub fn new(capacity: usize) -> Self {
    RingBuffer {
      storage: vec![None; capacity],
      read_idx: 1,
      write_idx: 0,
    }
  }

  fn push(&mut self, item: T) -> Result<(), Full> {
    if self.is_full() {
      return Err(Full);
    }

    self.storage[self.write_idx] = Some(item);
    self.write_idx = self.advance_idx(self.write_idx);
    Ok(())
  }

  fn pull(&self) -> Option<T> {
    if self.is_empty() {
      return None;
    }

    if let Some(item) = &self.storage[self.write_idx] {
      Some(item.clone())
    } else {
      None
    }
  }

  fn advance_idx(&self, idx: usize) -> usize {
    (idx + 1) % self.storage.len()
  }

  fn is_empty(&self) -> bool {
    todo!()
  }

  fn is_full(&self) -> bool {
    ((self.write_idx as i64) - (self.read_idx as i64)) + 1 == self.storage.len() as i64
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ring_buffer() {
    let mut buffer = RingBuffer::new(3);
    assert_eq!(buffer.push(1), Ok(()));
    assert_eq!(buffer.push(2), Ok(()));
    assert_eq!(buffer.push(3), Ok(()));
    assert_eq!(buffer.push(4), Err(Full));
    assert_eq!(buffer.pull(), Some(1));
    assert_eq!(buffer.pull(), Some(2));
    assert_eq!(buffer.pull(), Some(3));
    assert_eq!(buffer.pull(), None);
  }
}
