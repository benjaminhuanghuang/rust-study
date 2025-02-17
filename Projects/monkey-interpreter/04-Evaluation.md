# Evaluation

## Strategies of Evaluation

## Int

## Bool

## !true, -5

```rs
ExpressionNode::Prefix(prefix_exp) => {}

test_bang_operator

eval_prefix_expression

eval_minus_prefix_operator_expression
eval_bang_operator_expression
```

## 5+5, foo == bar

```rs
ExpressionNode::Infix(infix_exp) => {}
eval_infix_expression
eval_integer_infix_expression
eval_boolean_infix_expression
```

## If else

```rs
test_if_else_expression
```

## return

```rs
StatementNode::Return(statement) => {}
```

## error

```rs

pub enum Object {
  Error(String),
}

eval_prefix_expression
```

## let x = 4 \* 4;

```rs
 StatementNode::Let(statement) => {}
```
