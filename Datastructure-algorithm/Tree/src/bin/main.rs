

use tree::{BSTree, BSTreeNode};


fn main() {
  let mut bt_i32:BSTree<i32> = BSTree::new();

  let mut root = BSTreeNode::<i32>::new(0);
  let left = BSTreeNode::<i32>::new(1);
  let right = BSTreeNode::<i32>::new(2);

  root.left = Some(Box::new(left));
  root.right = Some(Box::new(right));
  bt_i32.root = Some(Box::new(root));
}