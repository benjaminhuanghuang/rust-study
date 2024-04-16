
use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;

struct ListNode<T>
{
    value :T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
    prev: Option<Rc<RefCell<ListNode<T>>>>
}


impl<T> ListNode<T> {
  fn new(value:T) -> Rc<RefCell<ListNode<T>>>{
      let pointer = Rc::new(RefCell::new(ListNode {
          value,
          next: None,
          prev: None,
      }));
      Rc::clone(&pointer)
  }
}

pub struct DoublLinkedList<T>{
  count: u32,
  first: Option<Rc<RefCell<ListNode<T>>>>,
  last: Option<Rc<RefCell<ListNode<T>>>>
}


impl<T> DoublLinkedList<T> {
  pub fn new() -> Self {
      DoublLinkedList {
          count: 0,
          first: None,
          last: None,
      }
  }

  pub fn list_count(&self) -> u32 {
      self.count
  }

  pub fn list_push(&mut self,value:T){
      let node = ListNode::new(value);
      if let Some(ref l) = self.last {
          let mut n = node.borrow_mut();
          n.next = Some(Rc::clone(&l));
          if let Some(ref p) = l.borrow().prev {
              n.prev = Some(Rc::clone(&p));
              p.borrow_mut().next = Some(Rc::clone(&node));
          };
          l.borrow_mut().prev = Some(Rc::clone(&node));
      };
      self.count++;
  }

  pub fn list_unshift(&mut self, value:T){
      let node = ListNode::new(value);
      if let Some(ref f) = self.first {
          let mut n = node.borrow_mut();
          n.prev = Some(Rc::clone(&f));
          if let Some(ref p) = f.borrow().next {
              n.next = Some(Rc::clone(&p));
              p.borrow_mut().prev = Some(Rc::clone(&node));
          };
          f.borrow_mut().next = Some(Rc::clone(&node));
      };
      self.count = self.count+1;
  }

  pub fn list_shift(&mut self) -> T {
      if self.count == 0 {
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

  pub fn list_pop(&mut self) -> T {
      if self.count == 0 {
          panic!("No Items for pop!");
      }
      let mut value = 0;
      let mut pointer_pnext = None;
      if let Some(ref l) = self.last {
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
      if self.count == 0 {
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

  pub fn list_last(&self) -> Option<T> {
      if self.count == 0 {
          panic!("No Items!");
      }
      let mut value = 0;
      if let Some(ref l) = self.last {
          if let Some(ref p) = l.borrow().prev {
              value = p.borrow().value;
          };
      }
      value
  }

  pub fn list_clear(&mut self){
      while self.count > 0 {
          self.list_pop();
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
