pub struct TreeNode<T> {
  pub val: T,
  pub left: Option<Box<TreeNode<T>>>,
  pub right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
  #[inline]
  pub fn new(val: T) -> Self {
      TreeNode {
          val,
          left: None,
          right: None,
      }
  }
}

trait TraitBinaryTree<T> {
  fn pre_order(&self);
  fn in_order(&self);
  fn pos_order(&self);
}


pub struct BinaryTree<T> {
  pub root: Option<Box<TreeNode<T>>>
}


impl<T> BinaryTree<T> {
  pub fn new() -> Self {
      BinaryTree { root: None }
  }
}




