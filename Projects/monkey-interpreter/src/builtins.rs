use crate::{evaluator::NULL, object::Object};
pub struct Builtins;

impl Builtins {
  pub fn all_builtins(&self) -> Vec<(String, Object)> {
    vec![
      (String::from("len"), Object::Builtin(b_len)),
      (String::from("first"), Object::Builtin(b_first)),
      (String::from("last"), Object::Builtin(b_last)),
      (String::from("rest"), Object::Builtin(b_rest)),
      (String::from("push"), Object::Builtin(b_push)),
    ]
  }
}

fn b_len(args: Vec<Object>) -> Object {
  if args.len() != 1 {
    return Object::Error(format!(
      "wrong number of arguments. got={}, want=1",
      args.len()
    ));
  }

  return match &args[0] {
    Object::StringObj(str_literal) => Object::Integer(str_literal.len() as i64),
    Object::Array(arr) => Object::Integer(arr.len() as i64),
    other => Object::Error(format!(
      "argument to `len` not supported, got {}",
      other.object_type()
    )),
  };
}

fn b_first(args: Vec<Object>) -> Object {
  if args.len() != 1 {
    return Object::Error(format!(
      "wrong number of arguments. got={}, want=1",
      args.len()
    ));
  }

  if args[0].object_type() != "ARRAY" {
    return Object::Error(format!(
      "argument to `first` must be ARRAY, got {}",
      args[0].object_type()
    ));
  }
  if let Object::Array(arr) = &args[0] {
    if arr.len() > 0 {
      return arr[0].clone();
    }
  }
  NULL
}

fn b_last(args: Vec<Object>) -> Object {
  if args.len() != 1 {
    return Object::Error(format!(
      "wrong number of arguments. got={}, want=1",
      args.len()
    ));
  }

  if args[0].object_type() != "ARRAY" {
    return Object::Error(format!(
      "argument to `first` must be ARRAY, got {}",
      args[0].object_type()
    ));
  }
  if let Object::Array(arr) = &args[0] {
    if arr.len() > 0 {
      return arr[arr.len() - 1].clone();
    }
  }
  NULL
}

fn b_rest(args: Vec<Object>) -> Object {
  if args.len() != 1 {
    return Object::Error(format!(
      "wrong number of arguments. got={}, want=1",
      args.len()
    ));
  }

  if args[0].object_type() != "ARRAY" {
    return Object::Error(format!(
      "argument to `first` must be ARRAY, got {}",
      args[0].object_type()
    ));
  }
  if let Object::Array(arr) = &args[0] {
    if arr.len() > 0 {
      return Object::Array(arr[1..].to_vec());
    }
  }
  NULL
}

/*
  push([1, 2, 3], 4) => [1, 2, 3, 4]
*/
fn b_push(args: Vec<Object>) -> Object {
  if args.len() != 2 {
    return Object::Error(format!(
      "wrong number of arguments. got={}, want=2",
      args.len()
    ));
  }

  if args[0].object_type() != "ARRAY" {
    return Object::Error(format!(
      "argument to `first` must be ARRAY, got {}",
      args[0].object_type()
    ));
  }
  if let Object::Array(arr) = &args[0] {
    let mut new_arr = arr.clone();
    new_arr.push(args[1].clone());
    return Object::Array(new_arr);
  }
  NULL
}
