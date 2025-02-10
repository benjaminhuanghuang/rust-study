use axum::{http::StatusCode, response::IntoResponse, Body};

pub mod jwt;
pub mod user;

pub enum ApiError {
  Auth(AuthError),
  Internal(anyhow::Error),
}
/*
 allows any type E that can be converted into anyhow::Error to be automatically converted into ApiError::Internal
*/
impl<E> From<E> for ApiError
where
  E: Into<anyhow::Error>, //// Restricts `E` to types that can convert into `anyhow::Error`
{
  fn from(err: E) -> Self {
    ApiError::Internal(err.into()) //// Convert `E` into `anyhow::Error` and wrap it in `ApiError::Internal`
  }
}

impl From<AuthError> for ApiError {
  fn from(err: AuthError) -> Self {
    ApiError::Auth(err)
  }
}

impl IntoResponse for ApiError {
  fn into_response(self) -> axum::response::Response {
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
  }
}
