/*
Defines a custom Error enum with multiple variants:
  Generic(String): Stores a generic error message.
  Static(&'static str): Stores a static error message.
  IO(std::io::Error): Wraps std::io::Error and allows automatic conversion.

*/
#[derive(thiserror::Error, Debug)]
pub enum Error {
  /// For starter, to remove as code matures.
  #[error("Generic error: {0}")]
  Generic(String),
  /// For starter, to remove as code matures.
  #[error("Static error: {0}")]
  Static(&'static str),

  #[error(transparent)]
  IO(#[from] std::io::Error),
}
