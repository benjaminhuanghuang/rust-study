use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

pub struct Claims {
  pub sub: String,
  pub exp: usize,
}

#[derive(Serialize, Deserialize)]
impl Claims {
  pub fn new(sub: String) -> Self {
    //let exp = System::now().add(Duration::days(1)).integer();
    let exp = SystemTime::now() + Duration::from_secs(15 * 60 * 60 * 24);
    let exp = exp
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap()
      .as_secs() as usize;

    Claims { sub, exp }
  }
}

pub enum AuthError {
  WrongCredentials,
  MissingCredentials,
  TokenCreation,
  InvalidToken,
}
