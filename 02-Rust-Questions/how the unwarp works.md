# unwrap

## unwrap for Option<T>

The Option enum represents a value that can either be present (Some(T)) or absent (None).

The implementation of unwrap for Option<T> looks like this:

```rs
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val, // Return the value if it's Some(T)
            None => panic!("called `Option::unwrap()` on a `None` value"), // Panic if it's None
        }
    }
}
```

## unwrap for Result<T, E>

The Result enum represents the outcome of an operation that can either be successful (Ok(T)) or fail (Err(E)).

```rs
impl<T, E: Debug> Result<T, E> {
    pub fn unwrap(self) -> T {
        match self {
            Ok(val) => val, // Return the value if it's Ok(T)
            Err(err) => panic!("called `Result::unwrap()` on an `Err` value: {:?}", err), // Panic with debug info if it's Err(E)
        }
    }
}
```

