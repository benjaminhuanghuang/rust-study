use crate::token::{lookup_ident, Token, TokenKind};

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
    self.skip_whitespace();

    let token = match self.ch {
      '=' => {
        if self.peek_char() == '=' {
          self.read_char();
          Token {
            kind: TokenKind::Eq,
            literal: "==".to_string(),
          }
        } else {
          Lexer::new_token(TokenKind::Assign, self.ch)
        }
      }
      ';' => Lexer::new_token(TokenKind::Semicolon, self.ch),
      '(' => Lexer::new_token(TokenKind::Lparen, self.ch),
      ')' => Lexer::new_token(TokenKind::Rparen, self.ch),
      '{' => Lexer::new_token(TokenKind::Lbrace, self.ch),
      '}' => Lexer::new_token(TokenKind::Rbrace, self.ch),
      ',' => Lexer::new_token(TokenKind::Comma, self.ch),
      '+' => Lexer::new_token(TokenKind::Plus, self.ch),
      '-' => Lexer::new_token(TokenKind::Minus, self.ch),
      '!' => {
        if self.peek_char() == '=' {
          self.read_char();
          Token {
            kind: TokenKind::NotEq,
            literal: "!=".to_string(),
          }
        } else {
          Lexer::new_token(TokenKind::Bang, self.ch)
        }
      }
      '*' => Lexer::new_token(TokenKind::Asterisk, self.ch),
      '/' => Lexer::new_token(TokenKind::Slash, self.ch),
      '<' => Lexer::new_token(TokenKind::Lt, self.ch),
      '>' => Lexer::new_token(TokenKind::Gt, self.ch),
      '\0' => Token {
        kind: TokenKind::Eof,
        literal: "".to_string(),
      },
      _ => {
        return if Lexer::is_letter(self.ch) {
          let literal = self.read_identifier();
          let kind = lookup_ident(&literal);
          Token { kind, literal }
        } else if self.ch.is_ascii_digit() {
          let literal = self.read_number();
          let kind = TokenKind::Int;
          Token { kind, literal }
        } else {
          Lexer::new_token(TokenKind::Illegal, self.ch)
        };
      }
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

  fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
  }

  fn read_identifier(&mut self) -> String {
    let mut identifier = String::new();

    while Lexer::is_letter(self.ch) {
      identifier.push(self.ch);
      self.read_char();
    }

    identifier
  }

  fn skip_whitespace(&mut self) {
    while self.ch.is_ascii_whitespace() {
      self.read_char();
    }
  }

  fn is_digit(ch: char) -> bool {
    ch.is_numeric()
  }

  fn read_number(&mut self) -> String {
    let mut number = String::new();

    while Lexer::is_digit(self.ch) {
      number.push(self.ch);
      self.read_char();
    }

    number
  }
  fn peek_char(&self) -> char {
    if self.read_position >= self.input.len() {
      '\0'
    } else {
      self.input[self.read_position]
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Lexer;
  use crate::token::{Token, TokenKind};

  #[test]
  fn test_next_token_single_character() {
    let input = "=+(){},;";

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
        kind: TokenKind::Comma,
        literal: ",".to_string(),
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
        "tests[{idx}] - token type wrong. expected {}, got {}",
        exp_token.kind, tok.kind
      );
      assert_eq!(
        tok.literal, exp_token.literal,
        "tests[{idx}] - token literal wrong. expected {}, got {}",
        exp_token.literal, tok.literal
      );
    }
  }

  #[test]
  fn test_next_token() {
    let input = r#"
      let five=5;
      let ten=10;
      let add = fn(x, y) {
        x + y;
      };  
      let result = add(five, ten);  
      !-/*5;
      5 < 10 > 5;
      if(5 < 10) {
        return true;
      } else {
        return false;
      }
      10==10;
      10!=9;
    "#;

    let expected = vec![
      // let five=5;
      Token {
        kind: TokenKind::Let,
        literal: "let".to_string(),
      },
      Token {
        kind: TokenKind::Ident,
        literal: "five".to_string(),
      },
      Token {
        kind: TokenKind::Assign,
        literal: "=".to_string(),
      },
      Token {
        kind: TokenKind::Int,
        literal: "5".to_string(),
      },
      Token {
        kind: TokenKind::Semicolon,
        literal: ";".to_string(),
      },
      // let ten=10;
      Token {
        kind: TokenKind::Let,
        literal: "let".to_string(),
      },
      Token {
        kind: TokenKind::Ident,
        literal: "ten".to_string(),
      },
      Token {
        kind: TokenKind::Assign,
        literal: "=".to_string(),
      },
      Token {
        kind: TokenKind::Int,
        literal: "10".to_string(),
      },
      Token {
        kind: TokenKind::Semicolon,
        literal: ";".to_string(),
      },
      /*
      let add = fn(x, y) {
        x + y;
      };
      */
      Token {
        kind: TokenKind::Let,
        literal: "let".to_string(),
      },
      Token {
        kind: TokenKind::Ident,
        literal: "add".to_string(),
      },
      Token {
        kind: TokenKind::Assign,
        literal: "=".to_string(),
      },
      Token {
        kind: TokenKind::Function,
        literal: "fn".to_string(),
      },
      Token {
        kind: TokenKind::Lparen,
        literal: "(".to_string(),
      },
      Token {
        kind: TokenKind::Ident,
        literal: "x".to_string(),
      },
      Token {
        kind: TokenKind::Comma,
        literal: ",".to_string(),
      },
      Token {
        kind: TokenKind::Ident,
        literal: "y".to_string(),
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
        kind: TokenKind::Ident,
        literal: "x".to_string(),
      },
      Token {
        kind: TokenKind::Plus,
        literal: "+".to_string(),
      },
      Token {
        kind: TokenKind::Ident,
        literal: "y".to_string(),
      },
      Token {
        kind: TokenKind::Semicolon,
        literal: ";".to_string(),
      },
      Token {
        kind: TokenKind::Rbrace,
        literal: "}".to_string(),
      },
      Token {
        kind: TokenKind::Semicolon,
        literal: ";".to_string(),
      },
      // let result = add(five, ten);
      Token {
        kind: TokenKind::Let,
        literal: "let".to_string(),
      },
      Token {
        kind: TokenKind::Ident,
        literal: "result".to_string(),
      },
      Token {
        kind: TokenKind::Assign,
        literal: "=".to_string(),
      },
      Token {
        kind: TokenKind::Ident,
        literal: "add".to_string(),
      },
      Token {
        kind: TokenKind::Lparen,
        literal: "(".to_string(),
      },
      Token {
        kind: TokenKind::Ident,
        literal: "five".to_string(),
      },
      Token {
        kind: TokenKind::Comma,
        literal: ",".to_string(),
      },
      Token {
        kind: TokenKind::Ident,
        literal: "ten".to_string(),
      },
      Token {
        kind: TokenKind::Rparen,
        literal: ")".to_string(),
      },
      Token {
        kind: TokenKind::Semicolon,
        literal: ";".to_string(),
      },
      // !-/*5;
      Token {
        kind: TokenKind::Bang,
        literal: "!".to_string(),
      },
      Token {
        kind: TokenKind::Minus,
        literal: "-".to_string(),
      },
      Token {
        kind: TokenKind::Slash,
        literal: "/".to_string(),
      },
      Token {
        kind: TokenKind::Asterisk,
        literal: "*".to_string(),
      },
      Token {
        kind: TokenKind::Int,
        literal: "5".to_string(),
      },
      Token {
        kind: TokenKind::Semicolon,
        literal: ";".to_string(),
      },
      // 5 < 10 > 5;
      Token {
        kind: TokenKind::Int,
        literal: "5".to_string(),
      },
      Token {
        kind: TokenKind::Lt,
        literal: "<".to_string(),
      },
      Token {
        kind: TokenKind::Int,
        literal: "10".to_string(),
      },
      Token {
        kind: TokenKind::Gt,
        literal: ">".to_string(),
      },
      Token {
        kind: TokenKind::Int,
        literal: "5".to_string(),
      },
      Token {
        kind: TokenKind::Semicolon,
        literal: ";".to_string(),
      },
      /*
      if(5 < 10) {
        return true;
      } else {
        return false;
      }
      */
      Token {
        kind: TokenKind::If,
        literal: "if".to_string(),
      },
      Token {
        kind: TokenKind::Lparen,
        literal: "(".to_string(),
      },
      Token {
        kind: TokenKind::Int,
        literal: "5".to_string(),
      },
      Token {
        kind: TokenKind::Lt,
        literal: "<".to_string(),
      },
      Token {
        kind: TokenKind::Int,
        literal: "10".to_string(),
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
        kind: TokenKind::Return,
        literal: "return".to_string(),
      },
      Token {
        kind: TokenKind::True,
        literal: "true".to_string(),
      },
      Token {
        kind: TokenKind::Semicolon,
        literal: ";".to_string(),
      },
      Token {
        kind: TokenKind::Rbrace,
        literal: "}".to_string(),
      },
      Token {
        kind: TokenKind::Else,
        literal: "else".to_string(),
      },
      Token {
        kind: TokenKind::Lbrace,
        literal: "{".to_string(),
      },
      Token {
        kind: TokenKind::Return,
        literal: "return".to_string(),
      },
      Token {
        kind: TokenKind::False,
        literal: "false".to_string(),
      },
      Token {
        kind: TokenKind::Semicolon,
        literal: ";".to_string(),
      },
      Token {
        kind: TokenKind::Rbrace,
        literal: "}".to_string(),
      },
      // 10==10;
      Token {
        kind: TokenKind::Int,
        literal: "10".to_string(),
      },
      Token {
        kind: TokenKind::Eq,
        literal: "==".to_string(),
      },
      Token {
        kind: TokenKind::Int,
        literal: "10".to_string(),
      },
      Token {
        kind: TokenKind::Semicolon,
        literal: ";".to_string(),
      },
      // 10!=9;
      Token {
        kind: TokenKind::Int,
        literal: "10".to_string(),
      },
      Token {
        kind: TokenKind::NotEq,
        literal: "!=".to_string(),
      },
      Token {
        kind: TokenKind::Int,
        literal: "9".to_string(),
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
        "tests[{idx}] - token type wrong. expected {}, got {}",
        exp_token.kind, tok.kind
      );
      assert_eq!(
        tok.literal, exp_token.literal,
        "tests[{idx}] - token literal wrong. expected {}, got {}",
        exp_token.literal, tok.literal
      );
    }
  }
}
