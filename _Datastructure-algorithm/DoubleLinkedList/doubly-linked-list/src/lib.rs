// Immutable outside, but can mutate interior
use std::cell::RefCell;
// Reference counting pointer
// you can drop something while weak references exist. This stops us from getting cyclical loops
us std::rc::{Rc, Weak};

#[derive(Debug)]
struct DbNode<T> {
  data: T,
  next: Option<Rc<RefCell<Node<T>>>>,
  prev: Option<Weak<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub str DbList<T>{
  first: Option<Rc<RefCell<Node<T>>>>,
  last: Option<Rc<RefCell<Node<T>>>>,
}


impl<T> DbList<T> {
  pub fn new()->Self {
    Self{
      last: None,
      first: None,
    }
  }


  pub fn push_front(&mut self, data:T) {
    match self.first.take(){
      Some(r) => {
        let new_front= Rc::new(RefCell::new(DbNode{
          data,
          next: Some(r.clone),
          prev:None
        }));
        let mut m = r.borrow_mut();
        m.prev = Some(Rc::downgrade(&new_front));
        self.first = Some(new_front);
      },
      None => {
        let new_front= Rc::new(RefCell::new(DbNode{
          data,
          next: None,
          prev:None
        }));

        self.first = Some(Rc::downgrade(&new_front));
        self.first = Some(new_data);
      }
    }

  }

  pub push_back(&mut self, data:T) {
    match self.last.take(){
      Some(r) => {
        let new_back= Rc::new(RefCell::new(DbNode{
          data,
          prev: Some(r.clone),
          next:None
        }));
        let st = Weak::upgrade(&r).unwrap();
        let mut m = r.borrow_mut();
        self.last = Some(Rc::downgrade(&new_back));
        m.next = Some(new_back);
      },
      None => {
        let new_front= Rc::new(RefCell::new(DbNode{
          data,
          next: None,
          prev:None
        }));

        self.first = Some(Rc::downgrade(&new_front));
        self.first = Some(new_data);
      }
    }

  }
}






#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let mut d1 = DbList::new();
    d1.push_front(6);
    d1.push_front(6);
    d1.push_back(6);
  }
}
