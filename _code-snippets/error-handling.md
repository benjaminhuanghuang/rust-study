

Option returns None or Some

Result returns Err or Some.


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