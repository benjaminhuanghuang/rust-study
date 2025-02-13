use crate::token::Token;

pub struct Lexer {
  input: Vec<char>,
}

impl Lexer {
  pub fn new(input: &str) -> Lexer {
    Lexer {
      input: input.chars().collect(),
    }
  }

  pub fn next_token(&self) -> Token {}
}

#[cfg(test)]
mod tests {
  use super::Lexer;
  use crate::token::{Token, TokenKind};

  #[test]
  fn test_next_token() {
    let input = "=+(){}.;";

    let expected = vec![
      Token {
        kind: TokenKind::Assign,
        literal: "=".to_string(),
      },
      Token {
        kind: TokenKind::Plus,
        literal: "+".to_string(),
      },
      Token {
        kind: TokenKind::Lparen,
        literal: "(".to_string(),
      },
      Token {
        kind: TokenKind::Rparen,
        literal: ")".to_string(),
      },
      Token {
        kind: TokenKind::Lbrace,
        literal: "{".to_string(),
      },
      Token {
        kind: TokenKind::Rbrace,
        literal: "}".to_string(),
      },
      Token {
        kind: TokenKind::Semicolon,
        literal: ";".to_string(),
      },
      Token {
        kind: TokenKind::Eof,
        literal: "".to_string(),
      },
    ];

    let lexer = Lexer::new(input);

    for (idx, exp_token) in expected.iter().enumerate() {
      let tok = lexer.next_token();

      assert_eq!(
        tok.kind, exp_token.kind,
        "tests[{idx}] - token type wrong. expected={}, got={}",
        exp_token.kind, tok.kind
      );
      assert_eq!(
        tok.literal, exp_token.literal,
        "tests[{idx}] - token literal wrong. expected={}, got={}",
        exp_token.literal, tok.literal
      );
    }
  }
}
