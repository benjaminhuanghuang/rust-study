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

#[derive(PartialOrd)]
struce BSTreeNode<T>: TreeNode<T>{

}

trait BinaryTree<T> {
  fn pre_order(&self);
  fn in_order(&self);
  fn pos_order(&self);
}

trait BinarySearchTree<T:PartialOrd>: BinaryTree<T> {
  fn insert(&mut self, value: T);
}
