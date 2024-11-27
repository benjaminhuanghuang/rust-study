# Data Conversion in Rust

- No Implicit Conversion: Unlike some languages (e.g., C++ or Python), Rust doesn’t perform implicit type conversions.
  For instance, you cannot assign a u32 to a u64 without explicitly converting it.
- Explicit Conversion: Rust requires you to state conversions explicitly using methods or operators.

## Using as Keyword

as can be used for basic type casting, such as:

```rs
let x: u32 = 42;
let y: u64 = x as u64; // Explicit conversion

// Casting pointers (*const T to *mut T).

```

Limitations: Lossy conversions (e.g., casting f64 to u8 that results in truncation or overflow) are not checked.

## Conversion Traits:

- From Trait
  for lossless conversion

```rs
use std::convert::From;

let num: i32 = i32::from(42u8);

let my_str = "hello";
let my_string = String::from(my_str);
```

- Into Trait
  Often used for generic programming.

```rs
fn convert<T: Into<String>>(input: T) {
    let stringified: String = input.into();
}
```

- TryFrom and TryInto Traits
  For fallible conversions that might fail (e.g., parsing a string into a number). They return a Result to handle errors safely.

```rs
use std::convert::TryFrom;

let parsed: Result<i32, _> = i32::try_from(256u16);
match parsed {
    Ok(val) => println!("Success: {}", val),
    Err(e) => println!("Error: {}", e),
}
```

## parse() and FromStr trait, to_string() and ToString trait

```rs
let num: i32 = "42".parse().expect("Not a number!");

let num = 42;
let str_num = num.to_string();
```

## format and Display trait

```ts
// The '_' indicates that the lifetime of the Formatter reference is inferred by the Rust compiler.
impl fmt::Display for i32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self)
    }
}
```

## Serialize/deserialize

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```
