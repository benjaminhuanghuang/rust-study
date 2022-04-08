## Option

Manages the possibility of nonexistent values

```
  enum Option<T> {
    None,
    Some<T>
  }
```


## Result
Used for recoveralbe errors, for example: file access, data validation, parsing strings
```
  enum Result<T, E> {
    Ok(T),
    Err(E)
  }
```

```
// Unwrapping a `Some` variant will extract the value wrapped.
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());
```


## ? operator
Similar to a match statement

For Result type
- Unwarps the value if Ok variant
- Returns an error if Err variant
```
  let f: Result<File, Error> = File::open("hello.text");




```
For Option type
- Returns a value is with the Some variant
- Returns nothing for the None variant
