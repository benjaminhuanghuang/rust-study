## Use from_u32

```
enum AtomicNumber {
    HYDROGEN = 1,
    HELIUM = 2,
    // ...
    IRON = 26,
}

impl AtomicNumber {
    fn from_u32(value: u32) -> AtomicNumber {
        match value {
            1 => AtomicNumber::HYDROGEN,
            2 => AtomicNumber::HELIUM,
            // ...
            26 => AtomicNumber::IRON,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

fn main() {
    let element = AtomicNumber::from_u32(26);
}

```

## num_enum

Cargo.toml

```
[dependencies]
num_enum = "0.5.1"
```

```
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(TryFromPrimitive)]   // create try_from for enum
#[repr(u32)]
enum AtomicNumber {
    HYDROGEN = 1,
    HELIUM = 2,
    // ...
    IRON = 26,
}

fn main() {
    let element = AtomicNumber::try_from(26u32);
    match element {
        Some(AtomicNumber::IRON) => println!("Beware of Rust!"),
        Some(_) => {},
        None => println!("Unknown atomic number")
    }
}
```
