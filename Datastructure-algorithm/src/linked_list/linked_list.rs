/*

Rc<T>	Shared ownership of nodes (avoids ownership issues). Without Rc, once the list goes out of scope, nodes would be dropped immediately when a single reference is lost.

RefCell<T>	Allows mutable access to next without violating borrowing rules.

A Box<T> is a single-owner heap allocation, which means we can’t have multiple references to the same node (like head and tail).
*/
use std::cell::RefCell;
use std::rc::Rc;

struct ListNode<T> {
  value: T,
  next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> ListNode<T> {
  fn new(value: T) -> Rc<RefCell<ListNode<T>>> {
    Rc::new(RefCell::new(ListNode { value, next: None }))
  }
}

pub struct LinkedList<T> {
  length: usize,
  head: Option<Rc<RefCell<ListNode<T>>>>,
  tail: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T: Clone> LinkedList<T> {
  pub fn new() -> Self {
    LinkedList {
      length: 0,
      head: None,
      tail: None,
    }
  }

  pub fn get_length(&self) -> usize {
    self.length
  }

  pub fn push(&mut self, value: T) {
    let node = ListNode::new(value);
    match self.tail.take() {
      Some(tail_node) => tail_node.borrow_mut().next = Some(node.clone()),
      None => self.head = Some(node.clone()),
    };
    self.tail = Some(node);
    self.length += 1;
  }

  pub fn unshift(&mut self, value: T) {
    let node = ListNode::new(value);
    node.borrow_mut().next = self.head.take();
    if self.tail.is_none() {
      self.tail = Some(node.clone());
    }
    self.head = Some(node);
    self.length += 1;
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.length == 0 {
      return None;
    }

    let mut current = self.head.clone();
    let mut prev: Option<Rc<RefCell<ListNode<T>>>> = None;
    while let Some(ref node) = current.clone() {
      if node.borrow().next.is_none() {
        let val = node.borrow().value.clone();
        if let Some(prev_node) = prev {
          prev_node.borrow_mut().next = None;
          self.tail = Some(prev_node);
        } else {
          self.head = None;
          self.tail = None;
        }
        self.length -= 1;
        return Some(val);
      }
      prev = current;
      current = node.borrow().next.clone();
    }
    None
  }

  pub fn shift(&mut self) -> Option<T> {
    self.head.take().map(|node| {
      let value = node.borrow().value.clone();
      self.head = node.borrow_mut().next.take();
      if self.head.is_none() {
        self.tail = None;
      }
      self.length -= 1;
      value
    })
  }

  pub fn get_first(&self) -> Option<T> {
    self.head.as_ref().map(|node| node.borrow().value.clone())
  }

  pub fn get_last(&self) -> Option<T> {
    self.tail.as_ref().map(|node| node.borrow().value.clone())
  }

  pub fn clear(&mut self) {
    self.head = None;
    self.tail = None;
    self.length = 0;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_push_pop() {
    let mut a: LinkedList<i32> = LinkedList::new();
    a.push(1);
    a.push(2);
    assert_eq!(a.pop(), Some(2));
    assert_eq!(a.pop(), Some(1));
  }

  #[test]
  fn test_shift() {
    let mut a: LinkedList<i32> = LinkedList::new();
    a.unshift(3);
    a.unshift(1);
    a.unshift(2);
    assert_eq!(a.shift(), Some(2));
    assert_eq!(a.shift(), Some(1));
  }

  #[test]
  fn test_shift_push() {
    let mut a = LinkedList::new();
    a.push(1);
    a.push(2);
    assert_eq!(a.shift(), Some(1));
    assert_eq!(a.shift(), Some(2));
  }

  #[test]
  fn test_clear() {
    let mut a = LinkedList::new();
    a.push(1);
    a.push(2);
    a.clear();
    assert_eq!(a.get_length(), 0);
  }

  #[test]
  fn test_first_last() {
    let mut a = LinkedList::new();
    a.push(1);
    a.push(2);
    a.push(3);
    assert_eq!(a.get_first(), Some(1));
    assert_eq!(a.get_last(), Some(3));
  }
}
