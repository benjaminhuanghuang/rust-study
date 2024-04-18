<<<<<<< HEAD
use mockall::automock;

#[automock] // create a mock obj named MockWeatherApi
=======
>>>>>>> a2f82b60ed9d1fcc77737a2c2719e0613b5fcf40
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
mod test {
  use super::*;

  #[test]
  #[should_panic(expected = "Zip code can not be blank")]
  fn test_weather_message_blank_zip() {
    let zip_code = "";
    let api = RealWeatherApi;
    create_message_weather(&api, zip_code);
  }

  #[test]
  fn test_weather_message_with_zip() {
    let zip_code = "1234";
    let mut mock_api = MockWeatherApi::new();
    mock_api
      .expect_get_weather()
      .once()
      .returning(|_| "is cloudy today!".to_string());

    let weather_message = create_message_weather(&mock_api, zip_code);

    assert_eq!(
      weather_message,
      "The weather in your area (Zip: 1234) is cloudy today!"
    )
  }
}
