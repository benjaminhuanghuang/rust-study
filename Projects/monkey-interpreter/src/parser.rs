use std::collections::HashMap;

use crate::{
  ast::{
    BlockStatement, Boolean, ExpressionNode, ExpressionStatement, FunctionLiteral, Identifier,
    IfExpression, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression, Program,
    ReturnStatement, StatementNode,
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

    parser.register_prefix(TokenKind::True, Self::parse_boolean);
    parser.register_prefix(TokenKind::False, Self::parse_boolean);

    parser.register_prefix(TokenKind::Lparen, Self::parse_grouped_expression); // (1 + 1)

    parser.register_prefix(TokenKind::If, Self::parse_if_expression); // (1 + 1)
    parser.register_prefix(TokenKind::Function, Self::parse_function_literal); // fn(x, y) { x + y; }

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

  fn parse_boolean(&mut self) -> Option<ExpressionNode> {
    Some(ExpressionNode::BooleanNode(Boolean {
      token: self.cur_token.clone(),
      value: self.curr_token_is(TokenKind::True),
    }))
  }

  fn parse_grouped_expression(&mut self) -> Option<ExpressionNode> {
    self.next_token();
    let exp = self.parse_expression(PrecedenceLevel::Lowest);
    if !self.expect_peek(TokenKind::Rparen) {
      return None;
    }

    exp
  }

  fn parse_if_expression(&mut self) -> Option<ExpressionNode> {
    let mut expression = IfExpression {
      token: self.cur_token.clone(),
      condition: Default::default(),
      consequence: Default::default(),
      alternative: None,
    };

    if !self.expect_peek(TokenKind::Lparen) {
      return None;
    }

    self.next_token();

    expression.condition = Box::new(
      self
        .parse_expression(PrecedenceLevel::Lowest)
        .expect("error parsing condition"),
    );

    if !self.expect_peek(TokenKind::Rparen) {
      return None;
    }

    if !self.expect_peek(TokenKind::Lbrace) {
      return None;
    }

    expression.consequence = self.parse_block_statement();
    if self.peek_token_is(TokenKind::Else) {
      self.next_token();
      if !self.expect_peek(TokenKind::Lbrace) {
        return None;
      }
      expression.alternative = Some(self.parse_block_statement());
    }

    Some(ExpressionNode::IfExpressionNode(expression))
  }

  fn parse_function_literal(&mut self) -> Option<ExpressionNode> {
    let mut literal = FunctionLiteral {
      token: self.cur_token.clone(),
      parameters: vec![],
      body: Default::default(),
    };

    if !self.expect_peek(TokenKind::Lparen) {
      return None;
    }

    literal.parameters = self
      .parse_function_parameters()
      .expect("error paring function params");

    if !self.expect_peek(TokenKind::Lbrace) {
      return None;
    }
    literal.body = self.parse_block_statement();

    Some(ExpressionNode::Function(literal))
  }

  fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
    let mut identifiers = vec![];

    if self.peek_token_is(TokenKind::Rparen) {
      self.next_token();
      return Some(identifiers);
    }

    self.next_token();

    let ident = Identifier {
      token: self.cur_token.clone(),
      value: self.cur_token.literal.clone(),
    };
    identifiers.push(ident);

    while self.peek_token_is(TokenKind::Comma) {
      self.next_token(); // skip comma
      self.next_token();
      let ident = Identifier {
        token: self.cur_token.clone(),
        value: self.cur_token.literal.clone(),
      };
      identifiers.push(ident);
    }

    if !self.expect_peek(TokenKind::Rparen) {
      return None;
    }

    Some(identifiers)
  }

  fn parse_block_statement(&mut self) -> BlockStatement {
    let mut block = BlockStatement {
      token: self.cur_token.clone(),
      statements: vec![],
    };

    self.next_token();

    while !self.curr_token_is(TokenKind::Rbrace) && !self.curr_token_is(TokenKind::Eof) {
      if let Some(statement) = self.parse_statement() {
        block.statements.push(statement);
      }
      self.next_token();
    }

    block
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

/*---------------------------------TESTS---------------------------------*/
#[cfg(test)]
mod tests {

  use std::any;

  use super::Parser;
  use crate::{
    ast::{ExpressionNode, Identifier, Node, StatementNode},
    lexer::Lexer,
    token::TokenKind,
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
    let prefix_tests: Vec<(&str, &str, Box<dyn any::Any>)> = vec![
      ("!5;", "!", Box::new(5)),
      ("-15;", "-", Box::new(15)),
      ("!true", "!", Box::new(true)),
      ("!false", "!", Box::new(false)),
    ];

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
              test_literal_expression(&prefix.right, test.2);
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
    let infix_tests: Vec<(&str, Box<dyn any::Any>, &str, Box<dyn any::Any>)> = vec![
      ("5+5;", Box::new(5), "+", Box::new(5)),
      ("5-5;", Box::new(5), "-", Box::new(5)),
      ("5*5;", Box::new(5), "*", Box::new(5)),
      ("5/5;", Box::new(5), "/", Box::new(5)),
      ("5>5;", Box::new(5), ">", Box::new(5)),
      ("5<5;", Box::new(5), "<", Box::new(5)),
      ("5==5;", Box::new(5), "==", Box::new(5)),
      ("5!=5;", Box::new(5), "!=", Box::new(5)),
      ("true == true", Box::new(true), "==", Box::new(true)),
      ("true != false", Box::new(true), "!=", Box::new(false)),
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
          let exp = expression_statement.expression.as_ref().unwrap();
          test_infix_expression(exp, Box::new(test.1), test.2.to_string(), Box::new(test.3));
        }
        other => panic!("statement not ExpressionStatement. got {:?}", other),
      }
    }
  }

  #[test]
  fn test_operator_precedence_parsing() {
    let tests = vec![
      ("-a * b", "((-a) * b)"),
      ("!-a", "(!(-a))"),
      ("a + b + c", "((a + b) + c)"),
      ("a + b - c", "((a + b) - c)"),
      ("a * b * c", "((a * b) * c)"),
      ("a * b / c", "((a * b) / c)"),
      ("a + b / c", "(a + (b / c))"),
      ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
      ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
      ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
      ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
      (
        "3 + 4 * 5 == 3 * 1 + 4 * 5",
        "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
      ),
      ("3>5 == false", "((3 > 5) == false)"),
      ("3<5 == true", "((3 < 5) == true)"),
      ("1+(2+3)+4", "((1 + (2 + 3)) + 4)"),
      ("(5+5)*2", "((5 + 5) * 2)"),
      ("2/(5+5)", "(2 / (5 + 5))"),
      ("-(5+5)", "(-(5 + 5))"),
      ("!(true == true)", "(!(true == true))"),
    ];
    for test in tests {
      let lexer = Lexer::new(test.0);
      let mut parser = Parser::new(lexer);
      let program = parser.parse_program().unwrap();
      check_parser_errors(&parser);

      let actual = program.print_string();
      assert_eq!(actual, test.1, "expected={}, got={}", test.1, actual);
    }
  }

  #[test]
  fn test_boolean_expression() {
    let input = r#"
      true;
      false;
    "#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();
    check_parser_errors(&parser);
    assert_eq!(
      program.statements.len(),
      2,
      "program has not enough statements. got {}",
      program.statements.len()
    );

    let expected_values = vec![(TokenKind::True, "true"), (TokenKind::False, "false")];
    for (idx, test) in expected_values.into_iter().enumerate() {
      match &program.statements[idx] {
        StatementNode::Expression(exp_stmt) => {
          assert!(exp_stmt.expression.is_some());
          let exp = exp_stmt.expression.as_ref().unwrap();

          match exp {
            ExpressionNode::BooleanNode(bool_expr) => {
              assert_eq!(
                bool_expr.value,
                test.0 == TokenKind::True,
                "boolean.value not {}. got {}",
                test.0 == TokenKind::True,
                bool_expr.value
              );
              assert_eq!(
                bool_expr.token_literal(),
                test.1,
                "boolean.token_literal not {}. got {}",
                test.1,
                bool_expr.token_literal()
              );
            }
            other => panic!("exp not Boolean. got {:?}", other),
          }
        }
        other => panic!(
          "program.statement[{}] not ExpressionStatement. got {:?}",
          idx, other
        ),
      }
    }
  }

  #[test]
  fn text_if_else_expression() {
    let input = r#"
      if(x < y) { x } else { y }
    "#;

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
      StatementNode::Expression(exp_stmt) => {
        assert!(exp_stmt.expression.is_some());
        let exp = exp_stmt.expression.as_ref().unwrap();

        match exp {
          ExpressionNode::IfExpressionNode(if_expr) => {
            test_infix_expression(
              &if_expr.condition,
              Box::new("x"),
              String::from("<"),
              Box::new("y"),
            );
            assert_eq!(
              if_expr.consequence.statements.len(),
              1,
              "consequence is not 1 statement. got {}",
              if_expr.consequence.statements.len()
            );

            assert_eq!(
              if_expr.alternative.as_ref().unwrap().statements.len(),
              1,
              "consequence is not 1 statement. got {}",
              if_expr.alternative.as_ref().unwrap().statements.len(),
            );

            match &if_expr.consequence.statements[0] {
              StatementNode::Expression(consequence) => {
                test_identifier(
                  consequence
                    .expression
                    .as_ref()
                    .expect("error parsing consequence"),
                  String::from("x"),
                );
              }
              other => panic!("consequence[0] not ExpressionStatement. got {:?}", other),
            }
            match &if_expr.alternative.as_ref().unwrap().statements[0] {
              StatementNode::Expression(consequence) => {
                test_identifier(
                  consequence
                    .expression
                    .as_ref()
                    .expect("error parsing alternative"),
                  String::from("y"),
                );
              }
              other => panic!("statement not ExpressionStatement. got {:?}", other),
            }
          }
          other => panic!("exp not IfExpression. got {:?}", other),
        }
      }
      other => panic!(
        "program.statement[0] not ExpressionStatement. got {:?}",
        other
      ),
    }
  }

  #[test]
  fn text_if_expression() {
    let input = r#"
      if(x < y) { x }
    "#;

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
      StatementNode::Expression(exp_stmt) => {
        assert!(exp_stmt.expression.is_some());
        let exp = exp_stmt.expression.as_ref().unwrap();

        match exp {
          ExpressionNode::IfExpressionNode(if_expr) => {
            test_infix_expression(
              &if_expr.condition,
              Box::new("x"),
              String::from("<"),
              Box::new("y"),
            );
            assert_eq!(
              if_expr.consequence.statements.len(),
              1,
              "consequence is not 1 statement. got {}",
              if_expr.consequence.statements.len()
            );

            match &if_expr.consequence.statements[0] {
              StatementNode::Expression(consequence) => {
                test_identifier(
                  consequence
                    .expression
                    .as_ref()
                    .expect("error parsing consequence"),
                  String::from("x"),
                );
              }
              other => panic!("consequence[0] not ExpressionStatement. got {:?}", other),
            }
            assert!(if_expr.alternative.is_none());
          }
          other => panic!("exp not IfExpression. got {:?}", other),
        }
      }
      other => panic!(
        "program.statement[0] not ExpressionStatement. got {:?}",
        other
      ),
    }
  }

  #[test]
  fn test_function_literal_parsing() {
    let input = r#"
        fn(x, y) { x + y; }
      "#;
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
      StatementNode::Expression(exp_stmt) => {
        assert!(exp_stmt.expression.is_some());
        let exp = exp_stmt.expression.as_ref().unwrap();

        match exp {
          ExpressionNode::Function(fn_literal) => {
            assert_eq!(
              fn_literal.parameters.len(),
              2,
              "function literal parameters wrong. want 2, got {}",
              fn_literal.parameters.len()
            );

            match &fn_literal.parameters[0] {
              Identifier { token, value, .. } => {
                assert_eq!(value, "x", "parameter[0] is not 'x'. got {}", value);
                assert_eq!(
                  token.literal, "x",
                  "parameter[0] is not 'x'. got {}",
                  token.literal
                );
              }
            }

            match &fn_literal.parameters[1] {
              Identifier { token, value, .. } => {
                assert_eq!(value, "y", "parameter[0] is not 'y'. got {}", value);
                assert_eq!(
                  token.literal, "y",
                  "parameter[0] is not 'y'. got {}",
                  token.literal
                );
              }
            }

            assert_eq!(
              fn_literal.body.statements.len(),
              1,
              "function body statements wrong. want 2, got {}",
              fn_literal.body.statements.len(),
            );

            match &fn_literal.body.statements[0] {
              StatementNode::Expression(expression) => {
                test_infix_expression(
                  expression
                    .expression
                    .as_ref()
                    .expect("error parsing expression"),
                  Box::new("x"),
                  String::from("+"),
                  Box::new("y"),
                );
              }
              other => panic!(
                "function body statement is not ExpressionStatement. got {:?}",
                other
              ),
            }
          }
          other => panic!("exp not FunctionLiteral. got {:?}", other),
        }
      }
      other => panic!(
        "program.statement[0] not ExpressionStatement. got {:?}",
        other
      ),
    }
  }
  /* ---------------------------------HELPERS---------------------------------*/
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
  fn test_identifier(exp: &ExpressionNode, value: String) {
    match exp {
      ExpressionNode::IdentifierNode(identifier) => {
        assert_eq!(
          identifier.value, value,
          "identifier.value not {}. got {}",
          value, identifier.value
        );
        assert_eq!(
          identifier.token_literal(),
          value,
          "identifier.token_literal not `{}`. got {}",
          value,
          identifier.token_literal()
        );
      }
      other => panic!("exp not Identifier. got {:?}", other),
    }
  }

  fn test_literal_expression(exp: &ExpressionNode, expected: Box<dyn any::Any>) {
    match expected.downcast_ref::<String>() {
      Some(exp_string) => test_identifier(exp, exp_string.to_string()),
      None => match expected.downcast_ref::<i64>() {
        Some(int_exp) => test_integer_literal(exp, int_exp.to_owned()),
        None => match expected.downcast_ref::<bool>() {
          Some(bool) => test_boolean_literal(exp, bool.to_owned()),
          None => (),
        },
      },
    }
  }
  fn test_boolean_literal(exp: &ExpressionNode, value: bool) {
    match exp {
      ExpressionNode::BooleanNode(bool_expr) => {
        assert_eq!(
          bool_expr.value, value,
          "boolean.value not {}. got {}",
          value, bool_expr.value
        );
        assert_eq!(
          bool_expr.token_literal(),
          format!("{}", value),
          "boolean.token_literal not {}. got {}",
          value,
          bool_expr.token_literal()
        );
      }
      other => panic!("exp not Boolean. got {:?}", other),
    }
  }
  fn test_infix_expression(
    exp: &ExpressionNode,
    left: Box<dyn any::Any>,
    operator: String,
    right: Box<dyn any::Any>,
  ) {
    match exp {
      ExpressionNode::Infix(infix_exp) => {
        test_literal_expression(&infix_exp.left, left);
        assert_eq!(
          infix_exp.operator, operator,
          "infix.operator is not {}. got {}",
          operator, infix_exp.operator
        );
        test_literal_expression(&infix_exp.right, right);
      }
      other => panic!("exp not InfixExpression. got {:?}", other),
    }
  }
}
