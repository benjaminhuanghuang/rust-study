# Rust: expression vs statement

- Statements are instructions that do something, they don’t return a value. 

- Expressions evaluate to a value, they return that value.

Rust is an expression-oriented language. This means that most things are expressions, and evaluate to some kind of value

## assignment
Assigning a value to a variable is a statement, it doesn’t return anything.
```
let num = 5;

let also_num = (let num = 5); // error!
```

## If
在Rust中if 构造不是 statement  是一个expression。
这种区别意味着Rust 中的if else 条件总是会 回一个值。 值可以是empty 类型的() 也可 是实际的值。

```
  let result = if 1== 2 {
    "Correct"
  } else {
    "Wrong"
  }

```

```
pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz" {
        "bar"
    }
    else {
        "baz"
    }
}
```