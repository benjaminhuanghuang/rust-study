


##  guard
match arms can include a guard,which is an additional check after the pattern match

```
fn parse_positive_int(val: &str) -> MyResult<usize> {
  match val.parse() {
    // If the parse succeeds and the parsed value n is greater than 0, return it as an Ok variant.
    Ok(n) if n > 0 => Ok(n),
    _ => Err(From::from(val)),
  }
}
```

equals to
```
fn parse_positive_int(val: &str) -> MyResult<usize> {
  match val.parse() {
  Ok(n) => {
    if n > 0 {
      Ok(n)
    } else {
      Err(From::from(val))
    }
  }
  _ => Err(From::from(val)),
  }
}
```
