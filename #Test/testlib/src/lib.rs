mod message_birthday;
mod message_name;
mod message_wether;

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

pub fn divide(a: i32, b: i32) -> i32 {
  if b == 0 {
    panic!("Division by zero is not allowed!")
  }

  a / b
}

pub fn check_even(num: i32) -> Result<(), String> {
  if num % 2 == 0 {
    Ok(())
  } else {
    Err(String::from("The number is not even"))
  }
}

pub struct Person {
  pub name: String,
  pub date_of_birth: chrono::NaiveDate,
  pub zip_code: String,
}
impl Person {
  pub fn create_greeting(&self) -> String {
    let name_message = message_name::create_message_name(&self.name);
    let birthday_message = message_birthday::create_message_birthday(self.date_of_birth);
    let api = message_wether::RealWeatherApi;
    let weather_message = message_wether::create_message_weather(&api, &self.zip_code);

    format!("{}{}{}", name_message, birthday_message, weather_message)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }

  #[test]
  #[should_panic(expected = "Division by zero is not allowed!")]
  fn test_divide_by_zero() {
    divide(10, 0);
  }

  #[test]
  fn test_check_even() -> Result<(), String> {
    let num = 5;
    let result = check_even(num);

    match result {
      Ok(_) => Ok(()),
      Err(e) => Err(format!("Testing {} failed with error: {}", num, e)),
    }
  }
}
