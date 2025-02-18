#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
  pub fn new() -> Self {
    LinkedList(None)
  }

  pub fn push_front(&mut self, data: T) {
    let t = self.0.take();
    self.0 = Some((data, Box::new(LinkedList(t))));
  }

  pub fn push_back(&mut self, data: T) {
    match self.0 {
      Some((_, ref mut child)) => child.push_back(data),
      None => self.push_front(data),
    }
  }

  pub fn front(&self) -> Option<&T> {
    match self.0 {
      Some((ref data, _)) => Some(data),
      None => None,
    }
  }

  pub fn len(&self) -> usize {
    match self.0 {
      Some((_, ref child)) => 1 + child.len(),
      None => 0,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_push_front() {
    let mut ll = LinkedList::new();
    ll.push_front(3);
    ll.push_front(2);
    ll.push_front(1);

    // Verify the front element is 1
    assert_eq!(ll.front(), Some(&1));

    // Verify the size of the list
    assert_eq!(ll.len(), 3);
  }
  #[test]
  fn test_push_back() {
    let mut ll = LinkedList::new();
    ll.push_back(3);
    ll.push_back(2);
    ll.push_back(1);

    // Check length
    assert_eq!(ll.len(), 3);
  }
}
