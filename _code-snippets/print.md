{} If an object implements the `Display` trait, then it can be formatted for user-facing output

{:?} to format the debug view of the arguments

```
  // Args does not implement Display trait
  println!("{:?}", std::env::args());
```

{:#?} to include newlines and indentations to help me read the output called pretty-printing

indicates the width of the field as six characters with the text aligned to the right
< for left-justified
and ^ for centered text

```
println!("{:>6}\t{}", line_num, line)
```

format 9 to 0009

```rs
  let file_name = format!("{:>04}", id);
```

## Print error to STDERR

```rs
eprintln!("{}", e);

eprint!("{}", e);
```

## pretty print

```
  println!("{:#?}", config);
```

## print debug

```
dbg! (debug)
```

## Format

```
let expected = format!("{}: .* [(]os error 2[)]", bad);
```
