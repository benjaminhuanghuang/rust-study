pub fn create_message_weather(zip_code: &str) -> String {
  if zip_code.trim().is_empty() {
    panic!("Zip code can not be blank")
  }

  // Call external API to get basic weather info
  let api_msg = "is sunny today!";

  format!(" The weather in your area (Zip: {}) {}", zip_code, api_msg)
}

#[cfg(test)]
mod test {}
