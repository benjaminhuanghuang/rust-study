use chrono::{Datelike, Local, NaiveDate};

pub fn create_message_birthday(date_of_birth: NaiveDate) -> String {
  let today = Local::now().naive_local().date();
  if date_of_birth.month() == today.month() && date_of_birth.day() == today.day() {
    " Happy Birthday!".to_string()
  } else {
    String::new()
  }
}

#[cfg(test)]
mod test {}
