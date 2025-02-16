use crate::ast::Program;
use crate::object::Object;

pub struct Evaluator {}

impl Evaluator {
  pub fn new() -> Self {
    Evaluator {}
  }

  fn eval_program(&self, program: ast::Program) -> Object {
    let mut result = Object::Null;

    for statement in program.statements {
      result = self.eval_statement(statement);
    }

    result
  }

  fn eval_statement(&self, statement: ast::Statement) -> Object {
    match statement {
      ast::Statement::ExpressionStatement(statement) => self.eval_expression(statement.expression),
      _ => Object::Null,
    }
  }

  fn eval_expression(&self, expression: Option<ExpressionNode>) -> Object {
    if let Some(exp) = expression {
      return match exp {
        ExpressionNode::IntegerLiteral(value) => Object::Integer(value),
        _ => Object::Null,
      };
    }
    Object::Null;
  }
}

/*------------------Tests ------------------- */
#[cfg(test)]
mod test {
  use super::*;
  use crate::object::Object;

  #[test]
  fn test_eval_integer_expression() {
    let tests = vec![
      ("5", 5),
      ("10", 10),
      ("-5", -5),
      ("-10", -10),
      ("5 + 5 + 5 + 5 - 10", 10),
      ("2 * 2 * 2 * 2 * 2", 32),
      ("-50 + 100 + -50", 0),
      ("5 * 2 + 10", 20),
      ("5 + 2 * 10", 25),
      ("20 + 2 * -10", 0),
      ("50 / 2 * 2 + 10", 60),
      ("2 * (5 + 10)", 30),
      ("3 * 3 * 3 + 10", 37),
      ("3 * (3 * 3) + 10", 37),
      ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for test in tests {
      let evaluated = test_eval(test.0);
      test_integer_object(evaluated, test.1);
    }
  }

  /*----------------HELPER----------------- */
  fn test_eval(input: &str) -> Object {
    let lexer = lexer::Lexer::new(input);
    let mut parser = parser::Parser::new(lexer);
    let program = parser.parse_program();

    let evaluator = Evaluator::new();
    eval_program(program.unwrap())
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
}
