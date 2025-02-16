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
