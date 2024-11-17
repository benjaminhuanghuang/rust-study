# Macro for testing

```
assert!(expression, "Message {}", result) - 如果表达式的值是 false 则 panic。


# arguments must implement the PartialOrd trait to be compared
assert_eq!(left, right) 
assert_ne!(left, right)


#[should_panic(expected = "Message")]       expected panic

unimplemented!();

```