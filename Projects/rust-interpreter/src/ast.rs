use crate::token::Token;

pub trait Node {
  fn token_literal(&self) -> String;
  fn print_string(&self) -> String;
}

#[derive(Debug)]
pub enum StatementNode {
  Let(LetStatement),
  Return(ReturnStatement),
  Expression(ExpressionStatement),
}

impl Node for StatementNode {
  fn token_literal(&self) -> String {
    return match self {
      Self::Let(let_statement) => let_statement.token_literal(),
      Self::Return(return_statement) => return_statement.token_literal(),
      Self::Expression(expression_statement) => expression_statement.token_literal(),
    };
  }

  fn print_string(&self) -> String {
    return match self {
      Self::Let(let_statement) => let_statement.print_string(),
      Self::Return(return_statement) => return_statement.print_string(),
      Self::Expression(expression_statement) => expression_statement.print_string(),
    };
  }
}

#[derive(Debug)]
pub enum ExpressionNode {
  IdentifierNode(Identifier),
  Integer(IntegerLiteral),
}

impl Node for ExpressionNode {
  fn token_literal(&self) -> String {
    return match self {
      Self::IdentifierNode(identifier) => identifier.token_literal(),
      Self::Integer(integer_literal) => integer_literal.token_literal(),
    };
  }

  fn print_string(&self) -> String {
    return match self {
      Self::IdentifierNode(identifier) => identifier.print_string(),
      Self::Integer(integer_literal) => integer_literal.print_string(),
    };
  }
}

pub struct Program {
  pub statements: Vec<StatementNode>,
}

impl Node for Program {
  fn token_literal(&self) -> String {
    return if self.statements.len() > 0 {
      match &self.statements[0] {
        StatementNode::Let(let_statement) => let_statement.token_literal(),
        StatementNode::Return(return_statement) => return_statement.token_literal(),
        StatementNode::Expression(expression_statement) => expression_statement.token_literal(),
      }
    } else {
      String::from("")
    };
  }

  fn print_string(&self) -> String {
    let mut out = String::new();

    for statement in self.statements.as_slice() {
      out.push_str(statement.print_string().as_str());
    }

    out
  }
}

#[derive(Debug)]
pub struct LetStatement {
  pub token: Token,
  pub name: Identifier,
  pub value: Option<ExpressionNode>,
}

impl Node for LetStatement {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn print_string(&self) -> String {
    let mut out = String::new();

    out.push_str(self.token_literal().as_str());
    out.push_str(" ");
    out.push_str(self.name.print_string().as_str());
    out.push_str(" = ");

    if let Some(value) = &self.value {
      out.push_str(value.print_string().as_str());
    }

    out.push_str(";");

    out
  }
}

#[derive(Debug, Default)]
pub struct Identifier {
  pub token: Token,
  pub value: String,
}

impl Node for Identifier {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn print_string(&self) -> String {
    self.value.clone()
  }
}

#[derive(Debug)]
pub struct ReturnStatement {
  pub token: Token,
  pub return_value: Option<ExpressionNode>,
}

impl Node for ReturnStatement {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn print_string(&self) -> String {
    let mut out = String::from("");

    out.push_str(self.token_literal().as_str());
    out.push_str(" ");

    if let Some(return_value) = &self.return_value {
      out.push_str(return_value.print_string().as_str());
    }

    out.push_str(";");

    out
  }
}

#[derive(Debug, Default)]
pub struct ExpressionStatement {
  pub token: Token,
  pub expression: Option<ExpressionNode>,
}

impl Node for ExpressionStatement {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn print_string(&self) -> String {
    let mut out = String::new();

    if let Some(expression) = &self.expression {
      out.push_str(expression.print_string().as_str());
    }

    out.push_str(";");

    out
  }
}
#[derive(Debug, Default)]
pub struct IntegerLiteral {
  pub token: Token,
  pub value: i64,
}

impl Node for IntegerLiteral {
  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn print_string(&self) -> String {
    self.token_literal()
  }
}

/*---------------------------------TESTS---------------------------------*/
#[cfg(test)]
mod tests {
  use crate::token::{Token, TokenKind};

  use super::{ExpressionNode, Identifier, LetStatement, Node, Program, StatementNode};

  #[test]
  fn test_print_string() {
    let program = Program {
      statements: vec![StatementNode::Let(LetStatement {
        token: Token {
          kind: TokenKind::Let,
          literal: String::from("let"),
        },
        name: Identifier {
          token: Token {
            kind: TokenKind::Ident,
            literal: String::from("myVar"),
          },
          value: String::from("myVar"),
        },
        value: Some(ExpressionNode::IdentifierNode(Identifier {
          token: Token {
            kind: TokenKind::Ident,
            literal: String::from("anotherVar"),
          },
          value: String::from("anotherVar"),
        })),
      })],
    };

    assert_eq!(
      program.print_string(),
      String::from("let myVar = anotherVar;"),
      "program.print_string() wrong. got={}",
      program.print_string()
    );
  }
}
