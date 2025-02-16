# Parsing

## Parsing Strategies

- Top-down: Create the root node first, then create the child

  - Recursive Descent / Pratt Parser
  - Early parsing
  - Predictive Parsing

- Bottom-up

## Parse let statement

create ast.rs

## Return statement

```rust
parse_statement

parse_return_statement
```

## Parsing an expression

```rust
ExpressionStatement
parse_expression_statement
```

## Prefix and infix function

```rust
type PrefixParseFn = fn(&mut Parser) -> Option<ExpressionNode>;
type InfixParseFn = fn(&mut Parser, ExpressionNode) -> Option<ExpressionNode>;

fn register_prefix()
fn register_infix()
```

## Parse int

```rust
IntegerLiteral

test_integer_expression

parse_integer_literal
```

## Parse prefix expression

```rust
PrefixExpression
parse_prefix_expression
```

## Parse infix expression

```rust
InfixExpression
test_parsing_infix_expression
parse_infix_expression

parse_expression   // Update
```

## Boolean

## Grouped Expression

```rust
parse_grouped_expression
```

## If

```rust
IfExpression

parse_if_expression
```

## Function

```rust
FunctionLiteral

pub enum ExpressionNode {
  Function(FunctionLiteral),
}

parser.register_prefix(TokenKind::Function, Self::parse_function_literal);

parse_function_literal
parse_function_parameters


test_function_literal_parsing
test_function_parameter_parsing
```

## Call

```rs
parser.register_infix(TokenKind::Lparen, Self::parse_call_expression);

parse_call_expression

TokenKind::Lparen => PrecedenceLevel::Call,

test_call_expression_parsing
```
