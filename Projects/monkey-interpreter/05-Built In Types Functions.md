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
// lexer
TokenKind::Lbracket => write!(f, "["),
TokenKind::Rbracket => write!(f, "]"),`

next_token(){
  '[' => Lexer::new_token(TokenKind::Lbracket, self.ch),
  ']' => Lexer::new_token(TokenKind::Rbracket, self.ch),
}

pub struct ArrayLiteral {
  pub token: Token,
  pub elements: Vec<ExpressionNode>,
}

// Parser
pub enum ExpressionNode {
  Array(ArrayLiteral),
}

parser.register_prefix(TokenKind::Lbracket, Self::parse_array_literal); // [1, 2, 3]

test_array_literal_parsing
```
