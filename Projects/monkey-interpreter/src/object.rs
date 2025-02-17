use std::{
  collections::HashMap,
  fmt::{Display, Formatter, Result},
};

use crate::{
  ast::{BlockStatement, Identifier, Node},
  builtins::{self, Builtins},
};

pub type BuiltinFunction = fn(Vec<Object>) -> Object;

#[derive(Debug, Clone)]
pub enum Object {
  Integer(i64),
  Boolean(bool),
  ReturnValue(Box<Object>),
  Error(String),
  Func(Function),
  StringObj(String),
  Builtin(BuiltinFunction),
  Null,
}

impl Object {
  pub fn object_type(&self) -> String {
    match self {
      Object::Integer(_) => String::from("INTEGER"),
      Object::Boolean(_) => String::from("BOOLEAN"),
      Object::ReturnValue(_) => String::from("RETURN_VALUE"),
      Object::Error(_) => String::from("ERROR"),
      Object::Func(_) => String::from("FUNCTION"),
      Object::StringObj(_) => String::from("STRING"),
      Object::Builtin(_) => String::from("BUILTIN"),
      Object::Null => String::from("NULL"),
    }
  }
}

impl Display for Object {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Object::Integer(value) => write!(f, "{}", value),
      Object::Boolean(value) => write!(f, "{}", value),
      Object::ReturnValue(value) => write!(f, "{}", *value),
      Object::Error(err) => write!(f, "{}", err),
      Object::Func(func) => {
        let mut out = String::from("");
        let mut params = vec![];
        for p in &func.parameters {
          params.push(p.print_string());
        }
        out.push_str("fn");
        out.push_str("(");
        out.push_str(&params.join(", ").as_str());
        out.push_str(") {\n");
        out.push_str(&func.body.print_string().as_str());
        out.push_str("\n}");
        write!(f, "{}", out)
      }
      Object::StringObj(value) => write!(f, "{}", value),
      Object::Builtin(_) => write!(f, "builtin function"),
      Object::Null => write!(f, "null"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Environment {
  pub store: HashMap<String, Object>,
  pub outer: Option<Box<Environment>>,
}

impl Environment {
  pub fn new_environment() -> Environment {
    let mut env_map = HashMap::new();
    Self::init_builtins(&mut env_map);
    Environment {
      store: env_map,
      outer: None,
    }
  }

  pub fn new_enclosed_environment(outer: Box<Environment>) -> Environment {
    let mut env_map = HashMap::new();
    Self::init_builtins(&mut env_map);
    Environment {
      store: env_map,
      outer: Some(outer),
    }
  }

  fn init_builtins(env: &mut HashMap<String, Object>) {
    let builtins_functions = Builtins;
    let builtins = builtins_functions.all_builtins();
    for (name, function) in builtins {
      env.insert(name, function);
    }
  }

  pub fn get(&self, name: String) -> Option<Object> {
    match self.store.get(name.as_str()) {
      Some(obj) => Some(obj.clone()),
      None => match &self.outer {
        Some(outer) => outer.get(name),
        None => None,
      },
    }
  }

  pub fn set(&mut self, name: String, value: Object) -> Option<Object> {
    self.store.insert(name.clone(), value);
    return self.get(name);
  }
}

#[derive(Debug, Clone)]
pub struct Function {
  pub parameters: Vec<Identifier>,
  pub body: BlockStatement,
  pub env: Environment,
}
