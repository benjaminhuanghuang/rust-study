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

## Array[index]

```rs
// lexer
IndexExpression

// Parser
pub enum ExpressionNode {
  Index(IndexExpression),
}

enum PrecedenceLevel {
  Index = 7,
}
TokenKind::Lbracket => PrecedenceLevel::Index,


parser.register_infix(TokenKind::Lbracket, Self::parse_index_expression); // array[1]

test_operator_precedence_parsing "a * [1, 2, 3, 4][b * c] * d",
```

## Evaluate Array

```rs
pub enum Object {
  Array(Vec<Object>),
}

eval_expression() {
  ExpressionNode::Array(array) => {}
  ExpressionNode::Index(index) => {}

}

eval_index_expression
eval_array_index_expression
```

## len(array), first(array)

## Hash

```rs
// Lexer
TokenKind {
  Colon
}

next_token() {
  ':' => Lexer::new_token(TokenKind::Colon, self.ch),
}


// Parsing
pub struct HashLiteral {
}

ExpressionNode {
  Hash(HashLiteral),
}

parser.register_infix(TokenKind::Lbrace, Self::parse_hash_literal); // { "key": "value" }

test_parsing_hash_literal_string_keys
```

## Hash Object
