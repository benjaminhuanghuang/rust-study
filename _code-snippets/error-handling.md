## Option

```
enum Option<T> {
  None,
  Some(T),
}
```

## Result returns Err or Some.

```
// 'static notation is telling the compiler that the error string will stay around for the entire runtime of the program.
fn error_check(check: bool) -> Result<i8, &'static str> {
if check == true {
Err("this is an error")
} else {
Ok(1)
}
}
```

check for an error using is_err:

```
result.is_err()
```

## Result

A Result value is one of two variants

```
  Ok(v)
  Err(e)
```

```
  u64::from_str(&arg)
```

## expect()

expect() to check a result. If the result is an Err(e), expect prints a message that
includes a description of e and exits the program
immediately. If the result is Ok(v), expect simply
returns v itself, which we are finally able to push onto the
end of our vector of numbers.

```
  u64::from_str(&arg).expect("error parsing argument")
```

## match

```

```

## ?
