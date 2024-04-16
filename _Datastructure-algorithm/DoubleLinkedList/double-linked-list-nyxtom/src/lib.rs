use std::{
  cell::RefCell,
  rc::{Rc, Weak},
};

pub struct DoubleLinkedListNode<T: Copy> {
  pub value: T,
  pub next: Option<Rc<RefCell<DoubleLinkedListNode<T>>>>,
  pub prev: Option<Weak<RefCell<DoubleLinkedListNode<T>>>>,
}

impl<T: Copy> DoubleLinkedListNode<T> {
  pub fn new(value: T) -> Self {
    DoubleLinkedListNode {
      value,
      next: None,
      prev: None,
    }
  }
}

impl<T: Copy> From<DoubleLinkedListNode<T>> for Option<Rc<RefCell<DoubleLinkedListNode<T>>>> {
  fn from(node: DoubleLinkedListNode<T>) -> Self {
    Some(Rc::new(RefCell::new(node)))
  }
}

type NodePtr<T> = Rc<RefCell<DoubleLinkedListNode<T>>>;

pub struct DoubleLinkedList<T: Copy> {
  head: Option<NodePtr<T>>,
  tail: Option<NodePtr<T>>,
}

impl<T: Copy> DoubleLinkedList<T> {
  pub fn new() -> Self {
    DoubleLinkedList {
      head: None,
      tail: None,
    }
  }

  pub fn push_front(&mut self, value: T) {
    let mut node = DoubleLinkedListNode::new(value);

    match &mut self.head.take() {
      None => {
        self.head = node.into();
        self.tail = self.head.clone();
      }
      Some(current_head) => {
        node.next = Some(current_head.clone());
        self.head = node.into();
        if let Some(h) = &self.head {
          current_head.borrow_mut().prev = Some(Rc::downgrade(&h))
        }
      }
    }
  }

  pub fn push_back(&mut self, value: T) {
    let mut node = DoubleLinkedListNode::new(value);

    match &mut self.tail.take() {
      None => {
        self.head = node.into();
        self.tail = self.head.clone();
      }
      Some(current_tail) => {
        node.prev = Some(Rc::downgrade(&current_tail));
        self.tail = node.into();
        current_tail.borrow_mut().next = self.tail.clone();
      }
    }
  }

  pub fn pop_back(&mut self) -> Option<T> {
    match &mut self.tail.take() {
      None => None,
      Some(tail) => {
        let mut tail = tail.borrow_mut();
        let prev = tail.prev.take();
        match prev {
          None => {
            self.head.take();
          }
          Some(prev) => {
            let prev = prev.upgrade();
            if let Some(prev) = prev {
              prev.borrow_mut().next = None;
              self.tail = Some(prev);
            }
          }
        };
        Some(tail.value)
      }
    }
  }

  pub fn pop_front(&mut self) -> Option<T> {
    match &mut self.head.take() {
      None => None,
      Some(head) => {
        let mut head = head.borrow_mut();
        let next = head.next.take();
        match next {
          None => {
            self.tail.take();
          }
          Some(next) => {
            next.borrow_mut().prev = None;
            self.head = Some(next);
          }
        };
        Some(head.value)
      }
    }
  }
}

impl<T: Copy> Drop for DoubleLinkedList<T> {
  fn drop(&mut self) {
    while let Some(_) = self.pop_back() {}
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut list = DoubleLinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);

    assert_eq!(list.pop_back(), Some(4));
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
  }

  #[test]
  fn it_works_front() {
    let mut list = DoubleLinkedList::new();

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);

    assert_eq!(list.pop_front(), Some(4));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
  }
}
