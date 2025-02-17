use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug, Default, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub literal: String,
}

#[derive(PartialEq, Debug, Default, Clone, Hash, Eq)]
pub enum TokenKind {
  #[default]
  Illegal,
  Eof,

  Ident,
  Int,

  Assign,
  Plus,
  Minus,
  Bang,
  Asterisk,
  Slash,

  Lt,
  Gt,
  Eq,
  NotEq,

  Comma,
  Semicolon,
  Colon,

  Lparen,
  Rparen,
  Lbrace,
  Rbrace,
  Lbracket,
  Rbracket,

  Function,
  Let,
  True,
  False,
  If,
  Else,
  Return,

  String,
}

impl Display for TokenKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      TokenKind::Illegal => write!(f, "Illegal"),
      TokenKind::Eof => write!(f, "Eof"),
      TokenKind::Ident => write!(f, "Ident"),
      TokenKind::Int => write!(f, "Int"),
      TokenKind::Assign => write!(f, "="),
      TokenKind::Plus => write!(f, "+"),
      TokenKind::Comma => write!(f, ":"),
      TokenKind::Semicolon => write!(f, ";"),
      TokenKind::Lparen => write!(f, "("),
      TokenKind::Rparen => write!(f, ")"),
      TokenKind::Lbrace => write!(f, "{{"),
      TokenKind::Rbrace => write!(f, "}}"),
      TokenKind::Function => write!(f, "Function"),
      TokenKind::Let => write!(f, "Let"),
      TokenKind::Bang => write!(f, "!"),
      TokenKind::Asterisk => write!(f, "*"),
      TokenKind::Slash => write!(f, "/"),
      TokenKind::Lt => write!(f, "<"),
      TokenKind::Gt => write!(f, ">"),
      TokenKind::Minus => write!(f, "-"),
      TokenKind::True => write!(f, "true"),
      TokenKind::False => write!(f, "false"),
      TokenKind::If => write!(f, "if"),
      TokenKind::Else => write!(f, "else"),
      TokenKind::Return => write!(f, "return"),
      TokenKind::Eq => write!(f, "=="),
      TokenKind::NotEq => write!(f, "!="),
      TokenKind::String => write!(f, "String"),
      TokenKind::Lbracket => write!(f, "["),
      TokenKind::Rbracket => write!(f, "]"),
      TokenKind::Colon => write!(f, ":"),
    }
  }
}

pub fn lookup_ident(identifier: &String) -> TokenKind {
  match identifier.as_str() {
    "fn" => TokenKind::Function,
    "let" => TokenKind::Let,
    "true" => TokenKind::True,
    "false" => TokenKind::False,
    "if" => TokenKind::If,
    "else" => TokenKind::Else,
    "return" => TokenKind::Return,
    _ => TokenKind::Ident,
  }
}
