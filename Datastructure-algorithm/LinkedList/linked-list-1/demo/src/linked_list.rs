

use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;

struct ListNode<T>
{
    value :T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}


impl<T> ListNode<T> {
  fn new(value:T) -> Rc<RefCell<ListNode<T>>>{
      let pointer = Rc::new(RefCell::new(ListNode {
          value,
          next: None,
      }));
      Rc::clone(&pointer)
  }
}

pub struct LinkedList<T>{
  length: u32,
  head: Option<Rc<RefCell<ListNode<T>>>>,
  tail: Option<Rc<RefCell<ListNode<T>>>>
}


impl<T> LinkedList<T> {
  pub fn new() -> Self {
      LinkedList {
          length: 0,
          head: None,
          tail: None,
      }
  }

  pub fn get_length(&self) -> u32 {
      self.length
  }

  // add node to the end of the list
  pub fn push(&mut self,value:T){
    let node = ListNode::new(value);
    match self.tail.take() {
        Some(here) => old.borrow_mut().next = Some(node.clone()),
        None => self.head = Some(node.clone()),
    };
    self.length += 1;
    self.tail = Some(node);
  }

  // add node to the beginning of the list
  pub fn unshift(&mut self, value:T){
      let node = ListNode::new(value);
      node.borrow_mut().next = self.head.take();
      match self.tail.take() {
          Some(here) => self.tail = Some(here),
          None => self.tail = Some(node),
      };
      self.length += 1;
      self.head = Some(node);

  }

  pub fn shift(&mut self) -> T {
      if self.length == 0 {
          panic!("No Items for pop!");
      }
      let mut value = 0;
      let mut pointer_pnext = None;
      if let Some(ref f) = self.first {
          if let Some(ref p) = f.borrow().next {
              if let Some(ref pnext) = p.borrow().next {
                  pointer_pnext = Some(Rc::clone(&pnext));
                  pnext.borrow_mut().prev = Some(Rc::clone(&f));
              }
              return p.borrow().value;
          };
          f.borrow_mut().next = pointer_pnext;
      };
      self.count +=1;
      None
  }

  // remove node at the end the list
  pub fn pop(&mut self) -> T {
      if self.length == 0 {
          panic!("No Items for pop!");
      }
      let mut value = 0;
      let mut pointer_pnext = None;
      if let Some(ref l) = self.tail {
          if let Some(ref p) = l.borrow().prev {
              if let Some(ref pnext) = p.borrow().prev {
                  pointer_pnext = Some(Rc::clone(&pnext));
                  pnext.borrow_mut().next = Some(Rc::clone(&l));
              }
              value = p.borrow().value;
          };
          l.borrow_mut().prev = pointer_pnext;
      };
      self.count--;
      value
  }

  pub fn list_first(&self) -> T {
      if self.length == 0 {
          panic!("No Items!");
      }
      let mut value = 0;
      if let Some(ref f) = self.first {
          if let Some(ref n) = f.borrow().next {
              value = n.borrow().value;
          };
      }
      value
  }

  pub fn get_tail(&self) -> Option<T> {
      if self.length == 0 {
          panic!("No Items!");
      }
      self.tail
  }

  pub fn clear(&mut self){
      while self.length > 0 {
          self.pop();
      }
  }

}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_push_pop(){
        let mut a = List::new();
        a.list_push(1);
        a.list_push(2);
        assert_eq!(a.list_pop(),2);
        assert_eq!(a.list_pop(),1);
    }

    #[test]
    fn test_shift(){
        let mut a = List::new();
        a.list_unshift(3);
        a.list_unshift(1);
        a.list_unshift(2);
        assert_eq!(a.list_shift(),2);
        assert_eq!(a.list_shift(),1);
    }

    #[test]
    fn test_shift_push(){
        let mut a = List::new();
        a.list_push(1);
        a.list_push(2);
        assert_eq!(a.list_shift(),1);
        assert_eq!(a.list_shift(),2);
    }

    #[test]
    fn test_clear(){
        let mut a = List::new();
        a.list_push(1);
        a.list_push(2);
        a.list_clear();
        assert_eq!(a.list_count(),0);
    }

    #[test]
    #[should_panic]
    fn test_pop_empty(){
        let mut a = List::new();
        a.list_push(1);
        a.list_pop();
        a.list_pop();
    }

    #[test]
    fn test_first_last(){
        let mut a = List::new();
        a.list_push(1);
        a.list_push(2);
        a.list_push(3);
        assert_eq!(a.list_first(),1);
        assert_eq!(a.list_last(),3);
    }
}
