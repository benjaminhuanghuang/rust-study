pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  LoginFail,
}

impl std::fmt::Display for Error {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(fmt, "Login Fail")
  }
}

impl IntoResponse for Error {
  fn into_response(self) -> http::Response<http::Body> {
    let (status, body) = match self {
      Error::LoginFail => (http::StatusCode::UNAUTHORIZED, "Login Fail"),
    };
    http::Response::builder()
      .status(status)
      .body(body.into())
      .unwrap()
  }
}
