##
Store a collection of mixed data without named fields

Distinguishable as unique data type

```
  struct Color(u8,u8, u8);

  let red = Color(255, 0, 0);

  println!("{}", red.0)

```