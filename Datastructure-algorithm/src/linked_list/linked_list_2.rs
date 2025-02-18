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

    assert_eq!(ll.0, Some((1, Box::new(LinkedList(Some((2, Box::new(LinkedList(Some((3, Box::new(LinkedList(None))))))))))));
  }

  #[test]
  fn test_push_back() {
    let mut ll = LinkedList::new();
    ll.push_back(3);
    ll.push_back(2);
    ll.push_back(1);

    assert_eq!(ll.0, Some((3, Box::new(LinkedList(Some((2, Box::new(LinkedList(Some((1, Box::new(LinkedList(None))))))))))));
  }
}
