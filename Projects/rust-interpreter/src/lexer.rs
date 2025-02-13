use crate::token::{Token, TokenKind};

pub struct Lexer {
  input: Vec<char>,
  position: usize,
  read_position: usize,
  ch: char,
}

impl Lexer {
  pub fn new(input: &str) -> Lexer {
    let mut lexer = Lexer {
      input: input.chars().collect(),
      position: 0,
      read_position: 0,
      ch: Default::default(), // '\0'
    };

    lexer.read_char();

    lexer
  }

  fn read_char(&mut self) {
    if self.read_position >= self.input.len() {
      self.ch = '\0';
    } else {
      self.ch = self.input[self.read_position];
    }
    self.position = self.read_position;
    self.read_position += 1;
  }

  pub fn next_token(&mut self) -> Token {
    let token = match self.ch {
      '=' => Lexer::new_token(TokenKind::Assign, self.ch),
      ';' => Lexer::new_token(TokenKind::Semicolon, self.ch),
      '(' => Lexer::new_token(TokenKind::Lparen, self.ch),
      ')' => Lexer::new_token(TokenKind::Rparen, self.ch),
      '{' => Lexer::new_token(TokenKind::Lbrace, self.ch),
      '}' => Lexer::new_token(TokenKind::Rbrace, self.ch),
      ',' => Lexer::new_token(TokenKind::Comma, self.ch),
      '+' => Lexer::new_token(TokenKind::Plus, self.ch),
      '\0' => Token {
        kind: TokenKind::Eof,
        literal: "".to_string(),
      },
      _ => Lexer::new_token(TokenKind::Illegal, self.ch),
    };

    self.read_char();

    token
  }

  fn new_token(kind: TokenKind, ch: char) -> Token {
    Token {
      kind,
      literal: ch.to_string(),
    }
  }
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

    let mut lexer = Lexer::new(input);

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
