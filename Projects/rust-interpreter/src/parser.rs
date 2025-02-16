use std::{collections::HashMap, f64::consts::E};

use crate::{
  ast::{
    ExpressionNode, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement,
    PrefixExpression, Program, ReturnStatement, StatementNode,
  },
  lexer::Lexer,
  token::{Token, TokenKind},
};

type PrefixParseFn = fn(&mut Parser) -> Option<ExpressionNode>;
type InfixParseFn = fn(&mut Parser, ExpressionNode) -> Option<ExpressionNode>;

#[derive(Debug, Copy, Clone)]
enum PrecedenceLevel {
  Lowest = 0,
  Equals = 1,      // ==
  LessGreater = 2, // > or <
  Sum = 3,         // +
  Product = 4,     // *
  Prefix = 5,      // -X or !X
  Call = 6,
}

fn precedence_map(kind: &TokenKind) -> PrecedenceLevel {
  return match kind {
    TokenKind::Eq => PrecedenceLevel::Equals,
    TokenKind::NotEq => PrecedenceLevel::Equals,
    TokenKind::Lt => PrecedenceLevel::LessGreater,
    TokenKind::Gt => PrecedenceLevel::LessGreater,
    TokenKind::Plus => PrecedenceLevel::Sum,
    TokenKind::Minus => PrecedenceLevel::Sum,
    TokenKind::Slash => PrecedenceLevel::Product,
    TokenKind::Asterisk => PrecedenceLevel::Product,
    TokenKind::Lparen => PrecedenceLevel::Call,
    _ => PrecedenceLevel::Lowest,
  };
}
pub struct Parser {
  lexer: Lexer,
  cur_token: Token,
  peek_token: Token,
  errors: Vec<String>,
  prefix_parse_fns: HashMap<TokenKind, PrefixParseFn>,
  infix_parse_fns: HashMap<TokenKind, InfixParseFn>,
}

impl Parser {
  pub fn new(lexer: Lexer) -> Self {
    let mut parser = Parser {
      lexer,
      cur_token: Default::default(),
      peek_token: Default::default(),
      errors: vec![],
      prefix_parse_fns: HashMap::new(),
      infix_parse_fns: HashMap::new(),
    };

    parser.register_prefix(TokenKind::Ident, Self::parse_identifier);
    parser.register_prefix(TokenKind::Int, Self::parse_integer_literal);
    parser.register_prefix(TokenKind::Bang, Self::parse_prefix_expression); // !x
    parser.register_prefix(TokenKind::Minus, Self::parse_prefix_expression); // -x

    parser.register_infix(TokenKind::Plus, Self::parse_infix_expression); // 1 + 1
    parser.register_infix(TokenKind::Minus, Self::parse_infix_expression); // 1 - 1
    parser.register_infix(TokenKind::Slash, Self::parse_infix_expression); // 1 / 1
    parser.register_infix(TokenKind::Asterisk, Self::parse_infix_expression); // 1 * 1
    parser.register_infix(TokenKind::Eq, Self::parse_infix_expression); // 1 == 1
    parser.register_infix(TokenKind::NotEq, Self::parse_infix_expression); // 1 != 1
    parser.register_infix(TokenKind::Lt, Self::parse_infix_expression); // 1 > 1
    parser.register_infix(TokenKind::Gt, Self::parse_infix_expression); // 1 < 1

    parser.next_token();
    parser.next_token();

    parser
  }

  fn parse_identifier(&mut self) -> Option<ExpressionNode> {
    Some(ExpressionNode::IdentifierNode(Identifier {
      token: self.cur_token.clone(),
      value: self.cur_token.literal.clone(),
    }))
  }

  fn parse_integer_literal(&mut self) -> Option<ExpressionNode> {
    let mut literal = IntegerLiteral {
      token: self.cur_token.clone(),
      value: Default::default(),
    };

    return match self.cur_token.literal.parse::<i64>() {
      Ok(value) => {
        literal.value = value;
        Some(ExpressionNode::Integer(literal))
      }
      Err(_) => {
        let msg = format!("could not parse {} as integer", self.cur_token.literal);
        self.errors.push(msg);
        None
      }
    };
  }

  fn parse_prefix_expression(&mut self) -> Option<ExpressionNode> {
    let mut expression = PrefixExpression {
      token: self.cur_token.clone(),
      operator: self.cur_token.literal.clone(),
      right: Default::default(),
    };
    self.next_token();

    match self.parse_expression(PrecedenceLevel::Prefix) {
      Some(expr) => expression.right = Box::new(expr),
      None => return None,
    }
    Some(ExpressionNode::Prefix(expression))
  }

  fn parse_infix_expression(&mut self, left: ExpressionNode) -> Option<ExpressionNode> {
    self.next_token();
    let mut expression = InfixExpression {
      token: self.cur_token.clone(),
      operator: self.cur_token.literal.clone(),
      left: Box::new(left),
      right: Default::default(),
    };
    let precedence = self.cur_precedence();
    self.next_token();
    match self.parse_expression(precedence) {
      Some(exp) => expression.right = Box::new(exp),
      None => return None,
    }
    Some(ExpressionNode::Infix(expression))
  }

  fn next_token(&mut self) {
    self.cur_token = self.peek_token.clone();
    self.peek_token = self.lexer.next_token();
  }

  pub fn parse_program(&mut self) -> Option<Program> {
    let mut program = Program { statements: vec![] };
    while self.cur_token.kind != TokenKind::Eof {
      if let Some(statement) = self.parse_statement() {
        program.statements.push(statement);
      }
      self.next_token();
    }
    Some(program)
  }

  fn parse_statement(&mut self) -> Option<StatementNode> {
    match self.cur_token.kind {
      TokenKind::Let => self.parse_let_statement(),
      TokenKind::Return => self.parse_return_statement(),
      _ => self.parse_expression_statement(),
    }
  }

  fn parse_let_statement(&mut self) -> Option<StatementNode> {
    let mut statement = LetStatement {
      token: self.cur_token.clone(),
      name: Default::default(),
      value: Default::default(),
    };
    return if !self.expect_peek(TokenKind::Ident) {
      None
    } else {
      statement.name = Identifier {
        token: self.cur_token.clone(),
        value: self.cur_token.literal.clone(),
      };
      if !self.expect_peek(TokenKind::Assign) {
        None
      } else {
        self.next_token();
        while !self.curr_token_is(TokenKind::Semicolon) {
          self.next_token();
        }
        Some(StatementNode::Let(statement))
      }
    };
  }

  fn parse_return_statement(&mut self) -> Option<StatementNode> {
    let statement = ReturnStatement {
      token: self.cur_token.clone(),
      return_value: Default::default(),
    };
    self.next_token();
    while !self.curr_token_is(TokenKind::Semicolon) {
      self.next_token();
    }
    Some(StatementNode::Return(statement))
  }

  fn parse_expression_statement(&mut self) -> Option<StatementNode> {
    let statement = ExpressionStatement {
      token: self.cur_token.clone(),
      expression: self.parse_expression(PrecedenceLevel::Lowest),
    };

    // 5 + 5, 5 * 5;
    if self.peek_token_is(TokenKind::Semicolon) {
      self.next_token();
    }
    Some(StatementNode::Expression(statement))
  }

  fn parse_expression(&mut self, precedence_level: PrecedenceLevel) -> Option<ExpressionNode> {
    let prefix = self.prefix_parse_fns.get(&self.cur_token.kind);

    if let Some(prefix_fn) = prefix {
      let mut left_exp = prefix_fn(self);
      while !self.peek_token_is(TokenKind::Semicolon)
        && ((precedence_level as u8) < (self.peek_precedence() as u8))
      {
        let infix_fn = self.infix_parse_fns.get(&self.peek_token.kind);
        if let Some(infix) = infix_fn {
          left_exp = infix(self, left_exp.expect("left expr should be present"))
        }
      }
      return left_exp;
    }
    self.no_prefix_parse_fn_error(self.cur_token.kind.clone());
    None
  }

  fn no_prefix_parse_fn_error(&mut self, kind: TokenKind) {
    let msg = format!("no prefix parse function for {:?} found", kind);
    self.errors.push(msg);
  }

  fn expect_peek(&mut self, kind: TokenKind) -> bool {
    if self.peek_token_is(kind.clone()) {
      self.next_token();
      true
    } else {
      self.peek_error(kind);
      false
    }
  }
  fn peek_token_is(&self, kind: TokenKind) -> bool {
    self.peek_token.kind == kind
  }
  fn curr_token_is(&self, kind: TokenKind) -> bool {
    self.cur_token.kind == kind
  }

  fn errors(&self) -> &Vec<String> {
    &self.errors
  }

  fn peek_error(&mut self, kind: TokenKind) {
    let msg = format!(
      "expected next token to be {:?}, got {:?} instead",
      kind, self.peek_token.kind
    );
    self.errors.push(msg);
  }

  fn register_prefix(&mut self, kind: TokenKind, func: PrefixParseFn) {
    self.prefix_parse_fns.insert(kind, func);
  }

  fn register_infix(&mut self, kind: TokenKind, func: InfixParseFn) {
    self.infix_parse_fns.insert(kind, func);
  }

  fn peek_precedence(&self) -> PrecedenceLevel {
    precedence_map(&self.peek_token.kind)
  }

  fn cur_precedence(&self) -> PrecedenceLevel {
    precedence_map(&self.cur_token.kind)
  }
}

#[cfg(test)]
mod tests {

  use super::Parser;
  use crate::{
    ast::{ExpressionNode, Node, StatementNode},
    lexer::Lexer,
  };

  #[test]
  fn test_let_statements() {
    let input = r#"
      let x = 5;
      let y = 10;
      let foobar = 838383;
    "#;
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    match program {
      Some(program) => {
        assert_eq!(
          program.statements.len(),
          3,
          "statements length is not 3. got {}",
          program.statements.len()
        );

        let expected = vec!["x", "y", "foobar"];
        for (idx, exp) in expected.into_iter().enumerate() {
          let statement = &program.statements[idx];
          test_let_statement(statement, exp);
        }
      }
      None => panic!("ParseProgram() returned None"),
    }
  }

  #[test]
  fn test_return_statements() {
    let input = r#"
    return 5;
    return 10;
    return 999999;
  "#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    check_parser_errors(&parser);

    match program {
      Some(program) => {
        assert_eq!(
          program.statements.len(),
          3,
          "statements length is not 3. got {}",
          program.statements.len()
        );

        for statement in program.statements {
          match statement {
            StatementNode::Return(return_statement) => {
              assert_eq!(
                return_statement.token_literal(),
                "return",
                "return_statement.token_literal not 'return'. got {}",
                return_statement.token_literal()
              );
            }
            other => panic!("statement not ReturnStatement. got {:?}", other),
          }
        }
      }
      None => panic!("ParseProgram() returned None"),
    }
  }

  #[test]
  fn test_identifier_expression() {
    let input = "foobar;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    check_parser_errors(&parser);

    assert_eq!(
      program.statements.len(),
      1,
      "program has not enough statements. got {}",
      program.statements.len()
    );

    match &program.statements[0] {
      StatementNode::Expression(expression_statement) => {
        assert!(expression_statement.expression.is_some());
        match expression_statement.expression.as_ref().unwrap() {
          ExpressionNode::IdentifierNode(identifier) => {
            assert_eq!(
              identifier.value, "foobar",
              "identifier.value not {}. got {}",
              "foobar", identifier.value
            );
            assert_eq!(
              identifier.token_literal(),
              "foobar",
              "identifier.token_literal not `{}`. got {}",
              "foobar",
              identifier.token_literal()
            );
          }
          other => panic!("expression not Identifier. got {:?}", other),
        }
      }
      other => panic!("statement not ExpressionStatement. got {:?}", other),
    }
  }

  #[test]
  fn test_integer_expression() {
    let input = "5;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    check_parser_errors(&parser);

    assert_eq!(
      program.statements.len(),
      1,
      "program has not enough statements. got {}",
      program.statements.len()
    );

    match &program.statements[0] {
      StatementNode::Expression(expression_statement) => {
        assert!(expression_statement.expression.is_some());
        match expression_statement.expression.as_ref().unwrap() {
          ExpressionNode::Integer(integer) => {
            assert_eq!(
              integer.value, 5,
              "integer.value not {}. got {}",
              5, integer.value
            );
            assert_eq!(
              integer.token_literal(),
              "5",
              "integer.token_literal not `{}`. got {}",
              "5",
              integer.token_literal()
            );
          }
          other => panic!("expression not Integer. got {:?}", other),
        }
      }
      other => panic!("statement not ExpressionStatement. got {:?}", other),
    }
  }

  #[test]
  fn test_parsing_prefix_expression() {
    let prefix_tests = vec![("!5;", "!", 5), ("-15;", "-", 15)];

    for test in prefix_tests {
      let lexer = Lexer::new(test.0);
      let mut parser = Parser::new(lexer);
      let program = parser.parse_program().unwrap();
      check_parser_errors(&parser);

      assert_eq!(
        program.statements.len(),
        1,
        "program has not enough statements. got {}",
        program.statements.len()
      );

      match &program.statements[0] {
        StatementNode::Expression(expression_statement) => {
          assert!(expression_statement.expression.is_some());
          match expression_statement.expression.as_ref().unwrap() {
            ExpressionNode::Prefix(prefix) => {
              assert_eq!(
                prefix.operator, test.1,
                "prefix.operator is not {}. got {}",
                test.1, prefix.operator
              );
              test_integer_literal(&prefix.right, test.2);
            }
            other => panic!("expression not PrefixExpression. got {:?}", other),
          }
        }
        other => panic!("statement not ExpressionStatement. got {:?}", other),
      }
    }
  }

  #[test]
  fn test_parsing_infix_expression() {
    let infix_tests = vec![
      ("5+5;", 5, "+", 5),
      ("5-5;", 5, "-", 5),
      ("5*5;", 5, "*", 5),
      ("5/5;", 5, "/", 5),
      ("5>5;", 5, ">", 5),
      ("5<5;", 5, "<", 5),
      ("5==5;", 5, "==", 5),
      ("5!=5;", 5, "!=", 5),
    ];

    for test in infix_tests {
      let lexer = Lexer::new(test.0);
      let mut parser = Parser::new(lexer);
      let program = parser.parse_program().unwrap();
      check_parser_errors(&parser);

      assert_eq!(
        program.statements.len(),
        1,
        "program has not enough statements. got {}",
        program.statements.len()
      );

      match &program.statements[0] {
        StatementNode::Expression(expression_statement) => {
          assert!(expression_statement.expression.is_some());
          match expression_statement.expression.as_ref().unwrap() {
            ExpressionNode::Infix(infix_expr) => {
              test_integer_literal(&infix_expr.left, test.1);
              assert_eq!(
                infix_expr.operator, test.2,
                "prefix.operator is not {}. got {}",
                test.2, infix_expr.operator
              );
              test_integer_literal(&infix_expr.right, test.3);
            }
            other => panic!("expression not InfixExpression. got {:?}", other),
          }
        }
        other => panic!("statement not ExpressionStatement. got {:?}", other),
      }
    }
  }

  fn test_let_statement(statement: &StatementNode, expected: &str) {
    assert_eq!(
      statement.token_literal(),
      "let",
      "statement.token_literal not 'let'. got {}",
      statement.token_literal()
    );

    match statement {
      StatementNode::Let(let_statement) => {
        assert_eq!(
          let_statement.name.value, expected,
          "let_statement.name.value not '{}'. got {}",
          expected, let_statement.name.value
        );

        assert_eq!(
          let_statement.name.token_literal(),
          expected,
          "let_statement.name.value not '{}'. got {}",
          expected,
          let_statement.name.token_literal(),
        );
      }
      other => {
        panic!("statement not LetStatement. got {:?}", other);
      }
    }
  }

  fn check_parser_errors(parser: &Parser) {
    let errors = parser.errors();
    if errors.len() == 0 {
      return;
    }

    for error in errors {
      eprintln!("parser error: {}", error);
    }
    panic!("parser error present");
  }

  fn test_integer_literal(exp: &ExpressionNode, value: i64) {
    match exp {
      ExpressionNode::Integer(integer) => {
        assert_eq!(
          integer.value, value,
          "integer.value not {}. got {}",
          value, integer.value
        );
        assert_eq!(
          integer.token_literal(),
          format!("{}", value),
          "int_exp.token_literal() not{}, got={}",
          value,
          integer.token_literal()
        );
      }
      other => panic!("exp not IntegerLiteral. got {:?}", other),
    }
  }
}
