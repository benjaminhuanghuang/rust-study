use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum Object {
  Integer(i64),
  Boolean(bool),
  Null,
}

impl Object {
  pub fn object_type(&self) -> String {
    match self {
      Object::Integer(_) => String::from("INTEGER"),
      Object::Boolean(_) => String::from("BOOLEAN"),
      Object::Null => String::from("NULL"),
    }
  }
}

impl Display for Object {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Object::Integer(value) => write!(f, "{}", value),
      Object::Boolean(value) => write!(f, "{}", value),
      Object::Null => write!(f, "null"),
    }
  }
}
