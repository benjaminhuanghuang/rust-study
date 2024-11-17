use chrono::{Duration, Local};
use testlib::Person;

#[test]
fn greeting_for_non_birthday() {
  let person = Person {
    name: "Alice".to_string(),
    date_of_birth: Local::now().naive_local().date() - Duration::days(1),
    zip_code: "12345".to_string(),
  };
  let greeting: String = person.create_greeting();
  assert!(greeting.contains("Hello, Alice"));
  assert!(!greeting.contains("Happy Birthday"));
  assert!(greeting.contains("12345"));
}

#[test]
fn greeting_for_birthday() {
  let person: Person = Person {
    name: "Bob".to_string(),
    date_of_birth: Local::now().naive_local().date(),
    zip_code: "12345".to_string(),
  };
  let greeting: String = person.create_greeting();
  assert!(greeting.contains("Hello, Bob"));
  assert!(!greeting.contains("Happy Birthday"));
  assert!(greeting.contains("12345"));
}
