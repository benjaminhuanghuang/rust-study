use std::num::ParseIntError;
use std::str::FromStr;
use ParsePersonError::{BadLen, Empty, NoName, ParseInt};

#[derive(Debug, PartialEq)]
struct Person {
  name: String,
  age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
  // Empty input string
  Empty,
  // Incorrect number of fields
  BadLen,
  // Empty name field
  NoName,
  // Wrapped error from parse::<usize>()
  ParseInt(ParseIntError),
}

impl FromStr for Person {
  type Err = ParsePersonError;
  fn from_str(s: &str) -> Result<Person, Self::Err> {
    if s.is_empty() {
      Err(Empty) // i.e., Err(ParsePersonError::Empty)
    } else {
      let p: Vec<&str> = s.split(',').collect();
      if p.len() != 2 {
        Err(BadLen)
      } else if p[0].len() == 0 {
        Err(NoName)
      } else {
        match p[1].parse::<usize>() {
          Ok(a) => Ok(Person {
            name: p[0].to_string(),
            age: a,
          }),
          Err(a) => Err(ParseInt(a)),
        }
      }
    }
  }
}

fn main() {
  let p = "Mark,20".parse::<Person>().unwrap();
  println!("{:?}", p);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty_input() {
    assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
  }
  #[test]
  fn good_input() {
    let p = "John,32".parse::<Person>();
    assert!(p.is_ok());
    let p = p.unwrap();
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 32);
  }
  #[test]
  fn missing_age() {
    assert!(matches!(
      "John,".parse::<Person>(),
      Err(ParsePersonError::ParseInt(_))
    ));
  }

  #[test]
  fn invalid_age() {
    assert!(matches!(
      "John,twenty".parse::<Person>(),
      Err(ParsePersonError::ParseInt(_))
    ));
  }

  #[test]
  fn missing_comma_and_age() {
    assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
  }

  #[test]
  fn missing_name() {
    assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
  }

  #[test]
  fn missing_name_and_age() {
    assert!(matches!(
      ",".parse::<Person>(),
      Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
    ));
  }

  #[test]
  fn missing_name_and_invalid_age() {
    assert!(matches!(
      ",one".parse::<Person>(),
      Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
    ));
  }

  #[test]
  fn trailing_comma() {
    assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
  }

  #[test]
  fn trailing_comma_and_some_string() {
    assert_eq!(
      "John,32,man".parse::<Person>(),
      Err(ParsePersonError::BadLen)
    );
  }
}
