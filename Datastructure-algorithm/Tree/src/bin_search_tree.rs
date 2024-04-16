pub struct BSTreeNode<T: PartialEq + PartialOrd> {
  value: T,
  pub left: Option<Box<BSTreeNode<T>>>,
  pub right: Option<Box<BSTreeNode<T>>>,
}

impl<T: PartialEq + PartialOrd> BSTreeNode<T> {
  #[inline]
  pub fn new(x: T) -> BSTreeNode<T> {
    BSTreeNode {
      value: x,
      left: None,
      right: None,
    }
  }
}

pub struct BSTree<T: PartialEq + PartialOrd> {
  pub root: Option<Box<BSTreeNode<T>>>,
}


impl<T: PartialEq + PartialOrd> BSTree<T> {
  pub fn new() -> Self {
    BSTree { root: None }
  }
}


