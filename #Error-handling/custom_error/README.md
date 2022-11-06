https://learning-rust.github.io/docs/e7.custom_error_types.html


## Error trait
creating custom error types the std::error::Error trait helps us to convert any type to an Err type
```
use std::fmt::{Debug, Display};

pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(Error + 'static)> { ... }
}
```

## From trait
Each crate uses their own error types. However, if we are using our own error type, we should convert those errors into our error type. For these conversions, we can use the standardized trait std::convert::From.
```
// traits inside Rust standard library core convert module/ std::convert
pub trait From<T>: Sized {
  fn from(_: T) -> Self;
}
```
For example String::from() function is used to create a String from &str data type

