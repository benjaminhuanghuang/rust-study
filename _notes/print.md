

{} If an object implements the `Display` trait, then it can be formatted for user-facing output

{:?} to format the debug view of the arguments
```
  // Args does not implement Display trait
  println!("{:?}", std::env::args());
```

{:#?} to include newlines and indentations to help me read the output called pretty-printing

