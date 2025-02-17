use crate::ast::{ExpressionNode, Program, StatementNode};
use crate::object::Object;

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

pub struct Evaluator {}

impl Evaluator {
  pub fn new() -> Self {
    Evaluator {}
  }

  pub fn eval_program(&self, program: Program) -> Object {
    let mut result = Object::Null;

    for statement in program.statements {
      result = self.eval_statement(statement);
    }

    result
  }

  fn eval_statement(&self, statement: StatementNode) -> Object {
    match statement {
      StatementNode::Expression(statement) => self.eval_expression(statement.expression),
      _ => Object::Null,
    }
  }

  fn eval_expression(&self, expression: Option<ExpressionNode>) -> Object {
    if let Some(exp) = expression {
      return match exp {
        ExpressionNode::Integer(int) => Object::Integer(int.value),
        ExpressionNode::BooleanNode(bool) => Self::native_bool_to_boolean_object(bool.value),
        ExpressionNode::Prefix(prefix_exp) => {
          let right = self.eval_expression(Some(*prefix_exp.right));
          return Self::eval_prefix_expression(prefix_exp.operator, right);
        }
        ExpressionNode::Infix(infix_exp) => {
          let left = self.eval_expression(Some(*infix_exp.left));
          let right = self.eval_expression(Some(*infix_exp.right));
          return Self::eval_infix_expression(infix_exp.operator, &left, &right);
        }
        _ => Object::Null,
      };
    }

    Object::Null
  }

  fn eval_prefix_expression(operator: String, right: Object) -> Object {
    match operator.as_str() {
      "!" => Self::eval_bang_operator_expression(right),
      "-" => Self::eval_minus_prefix_operator_expression(right),
      _ => NULL,
    }
  }

  fn eval_infix_expression(operator: String, left: &Object, right: &Object) -> Object {
    match (left, right, operator) {
      (Object::Integer(left), Object::Integer(right), operator) => {
        return Self::eval_integer_infix_expression(operator, *left, *right);
      }
      (Object::Boolean(left), Object::Boolean(right), operator) => {
        return Self::eval_boolean_infix_expression(operator, *left, *right);
      }
      _ => NULL,
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
  fn eval_boolean_infix_expression(operator: String, left: bool, right: bool) -> Object {
    match operator.as_str() {
      "==" => Self::native_bool_to_boolean_object(left == right),
      "!=" => Self::native_bool_to_boolean_object(left != right),
      _ => NULL,
    }
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
      _ => NULL,
    }
  }
  fn native_bool_to_boolean_object(bool: bool) -> Object {
    if bool {
      TRUE
    } else {
      FALSE
    }
  }
}

/*------------------Tests ------------------- */
#[cfg(test)]
mod test {
  use super::*;
  use crate::{lexer::Lexer, object::Object, parser::Parser};

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
  /*----------------HELPER----------------- */
  fn test_eval(input: &str) -> Object {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let evaluator = Evaluator::new();

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
}
