/*
struct Node<T> {
  value: T,
  left: Node<T>,
  right: Node<T>
}

Error: recursive type has infinite size
*/

use std::cell::RefCell;
use std::rc::Rc; // used then one valud owned by multi owner // can modify a value without mut.

struct TreeNode<T> {
  value: T,
  left: Option<Rc<RefCell<TreeNode<T>>>>,
  right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
  T: PartialEq + PartialOrd,
{
  #[inline]
  pub fn new(x: T) -> TreeNode<T> {
    TreeNode {
      value: x,
      left: None,
      right: None,
    }
  }
}

fn main() {
  let mut root = TreeNode::<u32>::new(0);
  let left = TreeNode::<u32>::new(1);
  let right = TreeNode::<u32>::new(2);
  root.left = Some(Rc::new(RefCell::new(left)));
  root.right = Some(Rc::new(RefCell::new(right)));
  println!("root is {:?}", root.value);

  if let Some(ref x) = root.left {
    println!("{:?}", x.borrow().value);
  }

  if let Some(ref mut x) = root.left {
    x.borrow_mut().value = 4
  }

  if let Some(ref mut x) = root.left {
    println!("left = {:?}", x.borrow().value);
  }
}