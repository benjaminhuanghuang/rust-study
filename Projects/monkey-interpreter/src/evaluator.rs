use std::collections::HashMap;
use std::ops::Deref;

use crate::ast::{
  BlockStatement, ExpressionNode, Identifier, IfExpression, Program, StatementNode,
};
use crate::object::{Environment, Function, HashPair, HashStruct, Hashable, Object};

pub const TRUE: Object = Object::Boolean(true);
pub const FALSE: Object = Object::Boolean(false);
pub const NULL: Object = Object::Null;

pub struct Evaluator {
  env: Environment,
}

impl Evaluator {
  pub fn new() -> Self {
    Evaluator {
      env: Environment::new_environment(),
    }
  }

  pub fn eval_program(&mut self, program: Program) -> Object {
    let mut result = NULL;

    for statement in program.statements {
      result = self.eval_statement(statement);

      if let Object::ReturnValue(val) = result {
        return *val;
      }

      if let Object::Error(_) = result {
        return result;
      }
    }

    result
  }

  fn eval_statement(&mut self, statement: StatementNode) -> Object {
    match statement {
      StatementNode::Expression(statement) => self.eval_expression(statement.expression),
      StatementNode::Return(statement) => {
        let val = self.eval_expression(statement.return_value);
        return Object::ReturnValue(Box::new(val));
      }
      StatementNode::Let(statement) => {
        let val = self.eval_expression(statement.value);
        if Self::is_error(&val) {
          return val;
        }
        self.env.set(statement.name.value, val).unwrap()
      }
      _ => NULL,
    }
  }

  fn eval_expression(&mut self, expression: Option<ExpressionNode>) -> Object {
    if let Some(exp) = expression {
      return match exp {
        ExpressionNode::Integer(int) => Object::Integer(int.value),
        ExpressionNode::BooleanNode(bool) => Self::native_bool_to_boolean_object(bool.value),
        ExpressionNode::Prefix(prefix_exp) => {
          let right = self.eval_expression(Some(*prefix_exp.right));
          if Self::is_error(&right) {
            return right;
          }
          return Self::eval_prefix_expression(prefix_exp.operator, right);
        }
        ExpressionNode::Infix(infix_exp) => {
          let left = self.eval_expression(Some(*infix_exp.left));
          if Self::is_error(&left) {
            return left;
          }
          let right = self.eval_expression(Some(*infix_exp.right));
          if Self::is_error(&right) {
            return right;
          }
          return Self::eval_infix_expression(infix_exp.operator, &left, &right);
        }
        ExpressionNode::IfExpressionNode(if_exp) => {
          return self.eval_if_expression(if_exp);
        }
        ExpressionNode::IdentifierNode(ident) => self.eval_identifier(ident),
        ExpressionNode::Function(fn_lit) => Object::Func(Function {
          parameters: fn_lit.parameters,
          body: fn_lit.body,
          env: self.env.clone(),
        }),
        ExpressionNode::Call(call_expr) => {
          let function = self.eval_expression(Some(call_expr.function.deref().clone()));
          if Self::is_error(&function) {
            return function;
          }
          let arg = self.eval_expressions(call_expr.arguments);
          if arg.len() == 1 && Self::is_error(&arg[0]) {
            return arg[0].clone();
          }
          self.apply_function(function, arg)
        }
        ExpressionNode::StringExp(string_literal) => Object::StringObj(string_literal.value),
        ExpressionNode::Array(array) => {
          let elements = self.eval_expressions(array.elements);
          if elements.len() == 1 && Self::is_error(&elements[0]) {
            return elements[0].clone();
          }
          Object::Array(elements)
        }
        ExpressionNode::Index(index_exp) => {
          let left = self.eval_expression(Some(*index_exp.left));
          if Self::is_error(&left) {
            return left;
          }
          let index = self.eval_expression(Some(*index_exp.index));
          if Self::is_error(&index) {
            return index;
          }

          self.eval_index_expression(left, index)
        }
        ExpressionNode::Hash(hash) => {
          let mut pairs = HashMap::new();
          for (k, v) in hash.pairs {
            let key = self.eval_expression(Some(k));
            if Self::is_error(&key) {
              return key;
            }
            let hash_key = match key.hash_key() {
              Ok(hash) => hash,
              Err(err) => return Object::Error(err.to_string()),
            };
            let value = self.eval_expression(Some(v));
            if Self::is_error(&value) {
              return value;
            }
            pairs.insert(hash_key, HashPair { key, value });
          }
          Object::HashObj(HashStruct { pairs })
        }
        _ => NULL,
      };
    }

    NULL
  }

  fn eval_index_expression(&mut self, left: Object, index: Object) -> Object {
    if left.object_type() == "ARRAY" && index.object_type() == "INTEGER" {
      return Self::eval_array_index_expression(left, index);
    }
    if left.object_type() == "HASH" {
      return Self::eval_hash_index_expression(left, index);
    }

    Object::Error(format!(
      "index operator not supported: {}",
      left.object_type()
    ))
  }
  fn eval_array_index_expression(array: Object, index: Object) -> Object {
    if let Object::Array(elements) = array {
      if let Object::Integer(i) = index {
        let max = elements.len() as i64 - 1;
        if i < 0 || i > max {
          return Object::Null;
        }
        return elements[i as usize].clone();
      }
    }
    Object::Null
  }

  fn eval_hash_index_expression(hash: Object, index: Object) -> Object {
    match hash {
      Object::HashObj(hash) => {
        let key = match index.hash_key() {
          Ok(key) => key,
          Err(err) => return Object::Error(err.to_string()),
        };

        let pair = match hash.pairs.get(&key) {
          Some(pair) => pair,
          None => return Object::Null,
        };
        return pair.value.clone();
      }
      _ => Object::Error(format!("unusable as hash key: {}", hash.object_type())),
    }
  }

  fn apply_function(&mut self, func: Object, args: Vec<Object>) -> Object {
    match func {
      Object::Func(function) => {
        let old_env = self.env.clone();
        let extended_env = Self::extended_function_env(function.clone(), args);
        self.env = extended_env;
        let evaluated = self.eval_block_expression(function.body);
        self.env = old_env;
        return Self::unwrap_return_value(evaluated);
      }
      Object::Builtin(builtin_fn) => builtin_fn(args),
      other => Object::Error(format!("not a function:{}", other.object_type())),
    }
  }

  fn extended_function_env(func: Function, args: Vec<Object>) -> Environment {
    let mut env = Environment::new_enclosed_environment(Box::new(func.env));
    for (i, param) in func.parameters.iter().enumerate() {
      env.set(param.value.clone(), args[i].clone());
    }
    env
  }

  fn unwrap_return_value(obj: Object) -> Object {
    match obj {
      Object::ReturnValue(val) => *val,
      _ => obj,
    }
  }
  fn eval_expressions(&mut self, expressions: Vec<ExpressionNode>) -> Vec<Object> {
    let mut result = vec![];

    for exp in expressions {
      let evaluated = self.eval_expression(Some(exp));
      if Self::is_error(&evaluated) {
        return vec![evaluated];
      }
      result.push(evaluated);
    }

    result
  }
  fn eval_prefix_expression(operator: String, right: Object) -> Object {
    match operator.as_str() {
      "!" => Self::eval_bang_operator_expression(right),
      "-" => Self::eval_minus_prefix_operator_expression(right),
      _ => Object::Error(format!(
        "unknown operator: {}{}",
        operator,
        right.object_type()
      )),
    }
  }

  fn eval_infix_expression(operator: String, left: &Object, right: &Object) -> Object {
    if left.object_type() != right.object_type() {
      return Object::Error(format!(
        "type mismatch: {} {} {}",
        left.object_type(),
        operator,
        right.object_type()
      ));
    }
    match (left, right, operator) {
      (Object::Integer(left), Object::Integer(right), operator) => {
        return Self::eval_integer_infix_expression(operator, *left, *right);
      }
      (Object::StringObj(left_str), Object::StringObj(right_str), operator) => {
        return match operator.as_str() {
          "+" => Object::StringObj(format!("{}{}", left_str, right_str)),
          _ => Object::Error(format!(
            "unknown operator: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
          )),
        };
      }
      (Object::Boolean(l), Object::Boolean(r), operator) => {
        return match operator.as_str() {
          "==" => Self::native_bool_to_boolean_object(l == r),
          "!=" => Self::native_bool_to_boolean_object(l != r),
          _ => Object::Error(format!(
            "unknown operator: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
          )),
        };
      }
      (left, right, operator) => Object::Error(format!(
        "unknown operator: {} {} {}",
        left.object_type(),
        operator,
        right.object_type()
      )),
    }
  }

  fn eval_integer_infix_expression(operator: String, left: i64, right: i64) -> Object {
    match operator.as_str() {
      "+" => Object::Integer(left + right),
      "-" => Object::Integer(left - right),
      "*" => Object::Integer(left * right),
      "/" => Object::Integer(left / right),
      "<" => Self::native_bool_to_boolean_object(left < right),
      ">" => Self::native_bool_to_boolean_object(left > right),
      "==" => Self::native_bool_to_boolean_object(left == right),
      "!=" => Self::native_bool_to_boolean_object(left != right),
      _ => NULL,
    }
  }

  fn eval_if_expression(&mut self, if_exp: IfExpression) -> Object {
    let condition = self.eval_expression(Some(*if_exp.condition));

    return if Self::is_truthy(condition) {
      self.eval_block_expression(if_exp.consequence)
    } else if let Some(alternative) = if_exp.alternative {
      self.eval_block_expression(alternative)
    } else {
      Object::Null
    };
  }

  fn is_truthy(obj: Object) -> bool {
    match obj {
      Object::Null => false,
      Object::Boolean(value) => value,
      _ => true,
    }
  }
  fn eval_identifier(&self, ident: Identifier) -> Object {
    let value = self.env.get(ident.value.clone());
    match value {
      Some(val) => val,
      None => Object::Error(format!("identifier not found: {}", ident.value)),
    }
  }

  fn eval_block_expression(&mut self, block: BlockStatement) -> Object {
    let mut result = Object::Null;

    for statement in block.statements {
      result = self.eval_statement(statement);

      if result.object_type() == "RETURN_VALUE" || result.object_type() == "ERROR" {
        return result;
      }
    }

    result
  }
  fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
      Object::Boolean(true) => FALSE,
      Object::Boolean(false) => TRUE,
      Object::Null => TRUE,
      _ => FALSE,
    }
  }
  fn eval_minus_prefix_operator_expression(right: Object) -> Object {
    match right {
      Object::Integer(value) => Object::Integer(-value),
      _ => Object::Error(format!("unknown operator: -{}", right.object_type())),
    }
  }
  fn native_bool_to_boolean_object(bool: bool) -> Object {
    if bool {
      TRUE
    } else {
      FALSE
    }
  }

  fn is_error(obj: &Object) -> bool {
    match obj {
      Object::Error(_) => true,
      _ => false,
    }
  }
}

/*------------------Tests ------------------- */
#[cfg(test)]
mod test {
  use std::any::{self};

  use super::*;
  use crate::{ast::Node, lexer::Lexer, object::Hashable, object::Object, parser::Parser};

  #[test]
  fn test_eval_integer_expression() {
    let tests = vec![
      ("5", 5),
      ("10", 10),
      ("-5", -5),
      ("-10", -10),
      ("5 + 5 + 5 + 5 - 10", 10),
      ("2 * 2 * 2 * 2 * 2", 32),
      // ("-50 + 100 + -50", 0),
      // ("5 * 2 + 10", 20),
      // ("5 + 2 * 10", 25),
      // ("20 + 2 * -10", 0),
      // ("50 / 2 * 2 + 10", 60),
      // ("2 * (5 + 10)", 30),
      // ("3 * 3 * 3 + 10", 37),
      // ("3 * (3 * 3) + 10", 37),
      // ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for test in tests {
      let evaluated = test_eval(test.0);
      test_integer_object(evaluated, test.1);
    }
  }

  #[test]
  fn test_eval_boolean_expression() {
    let tests = vec![
      ("true", true),
      ("false", false),
      ("1 < 2", true),
      ("1 > 2", false),
      ("1 < 1", false),
      ("1 > 1", false),
      ("1 == 1", true),
      ("1 != 1", false),
      ("1 == 2", false),
      ("1 != 2", true),
      ("true == true", true),
      ("false == false", true),
      ("true == false", false),
      ("true != false", true),
      ("false != true", true),
      ("(1 < 2) == true", true),
      ("(1 < 2) == false", false),
      ("(1 > 2) == true", false),
      ("(1 > 2) == false", true),
    ];
    for test in tests {
      let evaluated = test_eval(test.0);
      test_boolean_object(evaluated, test.1);
    }
  }

  #[test]
  fn test_bang_operator() {
    let tests = vec![
      ("!true", false),
      ("!false", true),
      ("!5", false),
      ("!!true", true),
      ("!!false", false),
      ("!!5", true),
    ];
    for test in tests {
      let evaluated = test_eval(test.0);
      test_boolean_object(evaluated, test.1);
    }
  }
  #[test]
  fn test_if_else_expression() {
    let tests = vec![
      ("if (true) { 10 }", 10),
      ("if (false) { 10 }", -999),
      ("if (1) { 10 }", 10),
      ("if (1 < 2) { 10 }", 10),
      ("if (1 > 2) { 10 }", -999),
      ("if (1 > 2) { 10 } else { 20 }", 20),
      ("if (1 < 2) { 10 } else { 20 }", 10),
    ];
    for test in tests {
      let evaluated = test_eval(test.0);
      if test.1 == -999 {
        test_null_object(evaluated);
      } else {
        test_integer_object(evaluated, test.1);
      }
    }
  }

  #[test]
  fn test_return_statement() {
    let tests = vec![
      ("return 10;", 10),
      ("return 10; 9;", 10),
      ("return 2 * 5; 9;", 10),
      ("9; return 2 * 5; 9;", 10),
      (
        "if (10 > 1) {
          if (10 > 1) {
            return 10;
          }
          return 1;
        }",
        10,
      ),
    ];
    for test in tests {
      let evaluated = test_eval(test.0);
      test_integer_object(evaluated, test.1);
    }
  }

  #[test]
  fn test_error_handling() {
    let tests = vec![
      ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
      ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
      ("-true", "unknown operator: -BOOLEAN"),
      ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
      ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
      (
        "if (10 > 1) { true + false; }",
        "unknown operator: BOOLEAN + BOOLEAN",
      ),
      (
        "
        if (10 > 1) {
          if (10 > 1) {
            return true + false;
          }
          return 1;
        }
        ",
        "unknown operator: BOOLEAN + BOOLEAN",
      ),
      ("foobar", "identifier not found: foobar"),
      (r#""Hello" - "World""#, "unknown operator: STRING - STRING"),
      (
        r#"{"name":"monkey"}[fn(x) { x }];"#,
        "unusable as hash key: FUNCTION",
      ),
    ];

    for tests in tests {
      let evaluated = test_eval(tests.0);
      match evaluated {
        Object::Error(err) => assert_eq!(err, tests.1),
        _ => panic!("no error object returned"),
      }
    }
  }

  #[test]
  fn test_let_statement() {
    let tests = vec![
      ("let a = 5; a;", 5),
      ("let a = 5 * 5; a;", 25),
      ("let a = 5; let b = a; b;", 5),
      ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
    ];

    for test in tests {
      let evaluated = test_eval(test.0);
      test_integer_object(evaluated, test.1);
    }
  }

  #[test]
  fn test_function_object() {
    let input = "fn(x) { x + 2; }";
    let evaluated = test_eval(input);
    match evaluated {
      Object::Func(func) => {
        assert_eq!(
          func.parameters.len(),
          1,
          "function has wrong parameters length. got {}",
          func.parameters.len()
        );
        assert_eq!(
          func.parameters[0].value, "x",
          "parameter is not 'x', got {}",
          func.parameters[0].value
        );
        assert_eq!(
          func.body.print_string(),
          "(x + 2)",
          "body is not '(x + 2)', got {}",
          func.body.print_string()
        );
      }
      _ => panic!("object is not function, got {:?}", evaluated),
    }
  }

  #[test]
  fn test_function_application() {
    let tests = vec![
      ("let identity = fn(x) { x; }; identity(5);", 5),
      ("let identity = fn(x) { return x; }; identity(5);", 5),
      ("let double = fn(x) { x * 2; }; double(5);", 10),
      ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
      ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
      ("fn(x) { x; }(5)", 5),
    ];

    for test in tests {
      let evaluated = test_eval(test.0);
      test_integer_object(evaluated, test.1);
    }
  }

  #[test]
  fn test_closures() {
    let input = "
    let newAdder = fn(x) {
      fn(y) { x + y };
    };
    let addTwo = newAdder(2);
    addTwo(2);
    ";
    let evaluated = test_eval(input);
    test_integer_object(evaluated, 4);
  }

  #[test]
  fn test_string_literal() {
    let input = "\"Hello World!\"";
    let evaluated = test_eval(input);
    match evaluated {
      Object::StringObj(value) => assert_eq!(
        value, "Hello World!",
        "object has wrong value. got {}, want {}",
        value, "Hello World!"
      ),
      other => panic!("object is not string, got {:?}", other),
    }
  }

  #[test]
  fn test_string_concatenation() {
    let input = r#"
        "Hello" + " " + "World!"
      "#;
    let evaluated = test_eval(input);
    match evaluated {
      Object::StringObj(value) => assert_eq!(
        value, "Hello World!",
        "object has wrong value. got {}, want {}",
        value, "Hello World!"
      ),
      other => panic!("object is not string, got {:?}", other),
    }
  }

  #[test]
  fn test_builtin_functions() {
    let tests: Vec<(&str, Box<dyn any::Any>)> = vec![
      (r#"len("")"#, Box::new(0_i64)),
      (r#"len("four")"#, Box::new(4_i64)),
      (r#"len("hello world")"#, Box::new(11_i64)),
      (
        r#"len(1)"#,
        Box::new(String::from("argument to `len` not supported, got INTEGER")),
      ),
      (
        r#"len("one", "two")"#,
        Box::new(String::from("wrong number of arguments. got=2, want=1")),
      ),
    ];

    for test in tests {
      let evaluated = test_eval(test.0);
      match test.1.downcast_ref::<i64>() {
        Some(expected) => test_integer_object(evaluated, *expected),
        None => match test.1.downcast_ref::<String>() {
          Some(expected) => match evaluated {
            Object::Error(err) => {
              assert_eq!(
                err, *expected,
                "Wrong error message, expected {}, got {}",
                *expected, err
              )
            }
            other => panic!("object is not error, got {:?}", other),
          },
          None => panic!("unexpected type"),
        },
      }
    }
  }

  #[test]
  fn test_array_literals() {
    let input = "[1, 2 * 2, 3 + 3]";
    let evaluated = test_eval(input);
    match evaluated {
      Object::Array(elements) => {
        assert_eq!(elements.len(), 3, "array has wrong num of elements");
        test_integer_object(elements[0].clone(), 1);
        test_integer_object(elements[1].clone(), 4);
        test_integer_object(elements[2].clone(), 6);
      }
      other => panic!("object is not array, got {:?}", other),
    }
  }

  #[test]
  fn test_array_index_expressions() {
    let tests: Vec<(&str, Box<dyn any::Any>)> = vec![
      ("[1, 2, 3][0]", Box::new(1_i64)),
      ("[1, 2, 3][1]", Box::new(2_i64)),
      ("[1, 2, 3][2]", Box::new(3_i64)),
      ("let i = 0; [1][i];", Box::new(1_i64)),
      ("[1, 2, 3][1 + 1];", Box::new(3_i64)),
      ("let myArray = [1, 2, 3]; myArray[2];", Box::new(3_i64)),
      (
        "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
        Box::new(6_i64),
      ),
      (
        "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
        Box::new(2_i64),
      ),
      ("[1, 2, 3][3]", Box::new(NULL)),
      ("[1, 2, 3][-1]", Box::new(NULL)),
    ];

    for test in tests {
      let evaluated = test_eval(test.0);
      match test.1.downcast_ref::<i64>() {
        Some(expected) => test_integer_object(evaluated, *expected),
        None => match test.1.downcast_ref::<i64>() {
          Some(expected) => test_integer_object(evaluated, *expected),
          None => test_null_object(evaluated),
        },
      }
    }
  }

  #[test]
  fn test_hash_literals() {
    let input = r#"
    let two = "two";
    {
      "one": 10 - 9,
      two: 1 + 1,
      "thr" + "ee": 6 / 2,
      4: 4,
      true: 5,
      false: 6
    }
    "#;
    let evaluated = test_eval(input);
    match evaluated {
      Object::HashObj(hash) => {
        let expected = vec![
          (Object::StringObj("one".to_string()).hash_key(), 1),
          (Object::StringObj("two".to_string()).hash_key(), 2),
          (Object::StringObj("three".to_string()).hash_key(), 3),
          (Object::Integer(4).hash_key(), 4),
          (TRUE.hash_key(), 5),
          (FALSE.hash_key(), 6),
        ];

        assert_eq!(
          hash.pairs.len(),
          expected.len(),
          "hash has wrong num of pairs. got {}",
          hash.pairs.len()
        );

        for (expected_key, expected_value) in expected {
          let pair = match hash.pairs.get(expected_key.as_ref().unwrap()) {
            Some(pair) => pair,
            None => panic!("no pair for given key in pairs"),
          };
          test_integer_object(pair.value.clone(), expected_value);
        }
      }
      other => panic!("object is not hash, got {:?}", other),
    }
  }

  #[test]
  fn test_hash_index_expressions() {
    let tests: Vec<(&str, Box<dyn any::Any>)> = vec![
      (r#"{"foo": 5}["foo"]"#, Box::new(5_i64)),
      (r#"{"foo": 5}["bar"]"#, Box::new(NULL)),
      (r#"let key= "foo"; {"foo": 5}[key]"#, Box::new(5_i64)),
      (r#"{}["foo"]"#, Box::new(NULL)),
      (r#"{5: 5}[5]"#, Box::new(5_i64)),
      (r#"{true: 5}[true]"#, Box::new(5_i64)),
      (r#"{false: 5}[false]"#, Box::new(5_i64)),
    ];

    for test in tests {
      let evaluated = test_eval(test.0);
      match test.1.downcast_ref::<i64>() {
        Some(expected) => test_integer_object(evaluated, *expected),
        None => test_null_object(evaluated),
      }
    }
  }

  /*----------------HELPER----------------- */
  fn test_eval(input: &str) -> Object {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let mut evaluator = Evaluator::new();

    evaluator.eval_program(program.unwrap())
  }

  fn test_integer_object(obj: Object, expected: i64) {
    match obj {
      Object::Integer(value) => assert_eq!(
        value, expected,
        "object has wrong value. got {}, want {}",
        value, expected
      ),
      _ => panic!("object is not integer, got {:?}", obj),
    }
  }

  fn test_boolean_object(obj: Object, expected: bool) {
    match obj {
      Object::Boolean(bool) => assert_eq!(
        bool, expected,
        "object has wrong value. got {}, want {}",
        bool, expected
      ),
      other => panic!("object is not bool, got {}", other),
    }
  }
  fn test_null_object(obj: Object) {
    match obj {
      Object::Null => assert!(true),
      _ => assert!(false),
    }
  }
}
