/*

https://www.youtube.com/watch?v=68356rTmqxY

*/

#[derive(Debug)]
pub struct ListNode<T> {
  pub value: T,
  pub next: Option<Box<ListNode<T>>>,
}

fn to_list<T>(vector: Vec<T>) -> Option<Box<ListNode<T>>> {
  let mut cur = None;
  for value in vector.into_iter().rev() {
    let mut new_node = ListNode { value, next: None };
    new_node.next = cur;
    cur = Some(Box::new(new_node));
  }
  cur
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_linked_list() {
    let list = to_list(vec![1, 2, 3]);
    println!("{:?}", list);
  }
}
