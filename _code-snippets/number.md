## float

```
  assert!(c <= 32::EPSILON)

  assert_eq!(f32::NAN == f32::NAN, false)
```

## str to number

Any type that implements the FromStr trait has a from_str() method that tries to parse a value of
that type from a string

```
use std::str::FromStr;

let mut numbers = Vec::new();

for arg in env::args().skip(1) {
  numbers.push(u64::from_str(&arg).expect("error parsing argument"));
}
```
