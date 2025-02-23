/*
`Vec<T>` Faster due to contiguous memory.
`LinkedList<T>` Slower but avoids reallocations.
*/

#[derive(Debug)]
pub struct StackNode<T> {
  elem: T,
  next: Option<Box<StackNode<T>>>,
}

#[derive(Debug)]
pub struct Stack<T> {
  top: Option<Box<StackNode<T>>>,
}

impl<T> Stack<T> {
  pub fn new() -> Stack<T> {
    Stack { top: None }
  }

  pub fn push(&mut self, elem: T) {
    let new_node = Some(Box::new(StackNode {
      elem,
      next: self.top.take(),
    }));
    self.top = new_node;
  }

  pub fn pop(&mut self) -> Option<T> {
    self.top.take().map(|node| {
      self.top = node.next;
      node.elem
    })
  }

  pub fn peek(&mut self) -> Option<&T> {
    self.top.as_ref().map(|node| &node.elem)
  }

  pub fn is_empty(&self) -> bool {
    self.top.is_none()
  }
}

#[cfg(test)]
mod test {
  use super::Stack;

  #[test]
  fn test_i32_operation() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));

    stack.push(5);
    stack.push(4);
    assert_eq!(stack.peek(), Some(&4));
  }

  #[test]
  fn test_string_operation() {
    let mut stack = Stack::new();
    stack.push("1");
    stack.push("2");
    stack.push("3");

    assert_eq!(stack.pop(), Some("3"));
    assert_eq!(stack.pop(), Some("2"));

    stack.push("5");
    stack.push("4");
    assert_eq!(stack.peek(), Some(&"4"));
  }
}
