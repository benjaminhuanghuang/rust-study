## String to int

```
  fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
      // If the parse succeeds and the parsed value n is greater than 0, return it as an Ok variant.
      Ok(n) if n > 0 => Ok(n),
      _ => Err(From::from(val)),
    }
  }
```

## convert &str to error

```
  Err(From::from(val))

  Err(val.into)

  Err(Inot::into(val))
```
