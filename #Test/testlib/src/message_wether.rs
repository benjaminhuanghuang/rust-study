pub trait WeatherApi {
  fn get_weather(&self, zip_code: &str) -> String;
}

pub struct RealWeatherApi;

impl WeatherApi for RealWeatherApi {
  fn get_weather(&self, _zip_code: &str) -> String {
    // Original logic to call external API
    "is sunny today!".to_string()
  }
}

pub fn create_message_weather(api: &dyn WeatherApi, zip_code: &str) -> String {
  if zip_code.trim().is_empty() {
    panic!("Zip code can not be blank")
  }

  // Call external API to get basic weather info
  let msg = api.get_weather(&zip_code);

  format!(" The weather in your area (Zip: {}) {}", zip_code, msg)
}

#[cfg(test)]
mod test {}
