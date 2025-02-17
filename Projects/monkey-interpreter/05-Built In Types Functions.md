# Built in Types

## String

```rs
fn test_next_token()

parse_string_literal

test_string_concatenation

eval_infix_expression
```

## len("string")

```rs
pub type BuiltinFunction = fn(Vec<Object>) -> Object;

#[derive(Debug, Clone)]
pub enum Object {
  BuildIn(BuiltinFunction),
}

test_builtin_functions

apply_function
```

## Array []

```rs
  TokenKind::Lbracket => write!(f, "["),
  TokenKind::Rbracket => write!(f, "]"),`

  next_token(){
    '[' => Lexer::new_token(TokenKind::Lbracket, self.ch),
    ']' => Lexer::new_token(TokenKind::Rbracket, self.ch),
  }
```
