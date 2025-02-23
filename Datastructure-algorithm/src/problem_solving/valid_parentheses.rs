/*
  Using stack to check if the parentheses are valid
*/
use crate::stack::stack_linkedlist::Stack;

pub fn is_valid(s: String) -> bool {
  let mut stack = Stack::new();
  for c in s.chars() {
    match c {
      '(' | '[' | '{' => stack.push(c),
      ')' => {
        if stack.pop() != Some('(') {
          return false;
        }
      }
      ']' => {
        if stack.pop() != Some('[') {
          return false;
        }
      }
      '}' => {
        if stack.pop() != Some('{') {
          return false;
        }
      }
      // Skip other characters
      _ => {}
    }
  }
  stack.is_empty()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valid_parentheses() {
    assert!(is_valid("(a)".to_string()));
    assert!(is_valid("(b)[]{c}".to_string()));
    assert!(is_valid("{[()()]}".to_string()));
    assert!(is_valid("".to_string()));
  }

  #[test]
  fn test_invalid_parentheses() {
    assert!(!is_valid("(]".to_string()));
    assert!(!is_valid("([)]".to_string()));
    assert!(!is_valid("{[(])}".to_string()));
    assert!(!is_valid("(".to_string()));
    assert!(!is_valid(")".to_string()));
  }
}
