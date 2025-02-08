// Stack structure
struct Stack<T> {
  data: Vec<T>,
}

impl<T> Stack<T> {
  // Create a new stack
  fn new() -> Self {
    Stack { data: Vec::new() }
  }

  // Function to add a value to the stack
  fn push(&mut self, item: T) {
    self.data.push(item);
  }

  // Function to remove a value from the stack
  fn pop(&mut self) -> Option<T> {
    self.data.pop()
  }

  // Check if the stack is empty
  fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  // Return the size of the stack
  fn size(&self) -> usize {
    self.data.len()
  }

  // Function to get the value at the top
  fn peek(&self) -> Option<&T> {
    self.data.last()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_stack_new() {
    let stack: Stack<i32> = Stack::new();
    assert!(stack.is_empty());
  }

  #[test]
  fn test_stack_push() {
    let mut stack = Stack::new();
    stack.push(1);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 1);
  }

  #[test]
  fn test_stack_pop() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
  }

  #[test]
  fn test_stack_is_empty() {
    let mut stack = Stack::new();
    assert!(stack.is_empty());
    stack.push(1);
    assert!(!stack.is_empty());
    stack.pop();
    assert!(stack.is_empty());
  }

  #[test]
  fn test_stack_size() {
    let mut stack = Stack::new();
    assert_eq!(stack.size(), 0);
    stack.push(1);
    assert_eq!(stack.size(), 1);
    stack.push(2);
    assert_eq!(stack.size(), 2);
    stack.pop();
    assert_eq!(stack.size(), 1);
  }

  #[test]
  fn test_stack_peek() {
    let mut stack = Stack::new();
    assert_eq!(stack.peek(), None);
    stack.push(1);
    assert_eq!(stack.peek(), Some(&1));
    stack.push(2);
    assert_eq!(stack.peek(), Some(&2));
    stack.pop();
    assert_eq!(stack.peek(), Some(&1));
  }
}
