use std::{
  collections::{hash_map::DefaultHasher, HashMap},
  fmt::{Display, Formatter},
  hash::{Hash, Hasher},
};

use crate::{
  ast::{BlockStatement, Identifier, Node},
  builtins::Builtins,
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
  Array(Vec<Object>),
  HashObj(HashStruct),
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
      Object::Array(_) => String::from("ARRAY"),
      Object::HashObj(_) => String::from("HASH"),
      Object::Null => String::from("NULL"),
    }
  }
}

impl Display for Object {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Integer(value) => write!(f, "{}", value),
      Self::Boolean(value) => write!(f, "{}", value),
      Self::ReturnValue(value) => write!(f, "{}", *value),
      Self::Error(err) => write!(f, "{}", err),
      Self::Func(func) => {
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
      Self::StringObj(value) => write!(f, "{}", value),
      Self::Builtin(_) => write!(f, "builtin function"),
      Self::Array(values) => {
        let mut out = String::from("[");
        let mut elements = vec![];
        for e in values {
          elements.push(e.to_string());
        }
        out.push_str(&elements.join(", "));
        out.push_str("]");
        write!(f, "{}", out)
      }
      Self::HashObj(hash) => {
        let mut out = String::from("{");
        let mut pairs = vec![];
        for (_, pair) in &hash.pairs {
          pairs.push(format!("{}: {}", pair.key, pair.value));
        }
        out.push_str("{");
        out.push_str(&pairs.join(", "));
        out.push_str("}");
        write!(f, "{}", out)
      }
      Self::Null => write!(f, "null"),
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

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct HashKey {
  pub object_type: String,
  pub value: i64,
}

impl Hash for HashKey {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.object_type.hash(state);
    self.value.hash(state);
  }
}

pub trait Hashable {
  fn hash_key(&self) -> Result<HashKey, String>;
}

impl Hashable for Object {
  fn hash_key(&self) -> Result<HashKey, String> {
    match &self {
      Object::Boolean(bool) => {
        let value = if *bool { 1 } else { 0 };
        Ok(HashKey {
          object_type: self.object_type(),
          value,
        })
      }
      Object::Integer(int) => Ok(HashKey {
        object_type: self.object_type(),
        value: *int,
      }),
      Object::StringObj(str) => {
        let mut hasher = DefaultHasher::new();
        str.hash(&mut hasher);
        Ok(HashKey {
          object_type: self.object_type(),
          value: hasher.finish() as i64,
        })
      }
      other => Err(format!("unusable as hash key: {}", other.object_type())),
    }
  }
}

#[derive(Debug, Clone)]
pub struct HashPair {
  pub key: Object,
  pub value: Object,
}
#[derive(Debug, Clone)]
pub struct HashStruct {
  pub pairs: HashMap<HashKey, HashPair>,
}
/*-------------------------------------------------------------
*                     TEST
-------------------------------------------------------------*/
#[cfg(test)]
mod test {
  use super::{Hashable, Object};

  #[test]
  fn test_string_hash_key() {
    let hello1 = Object::StringObj("Hello World".to_string());
    let hello2 = Object::StringObj("Hello World".to_string());
    let some_other = Object::StringObj("Some Other".to_string());

    assert_eq!(
      hello1.hash_key(),
      hello2.hash_key(),
      "strings with same content have different hash key"
    );

    assert_ne!(
      hello1.hash_key(),
      some_other.hash_key(),
      "strings with different content have same hash key"
    )
  }
}
