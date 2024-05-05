/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
  Info,
  Warning,
  Error,
}

/// "[ERROR]: the message"
pub fn log(level: LogLevel, message: &str) -> String {
  format!(
    "[{}]: {}",
    match level {
      LogLevel::Info => "INFO",
      LogLevel::Warning => "WARNING",
      LogLevel::Error => "ERROR",
    },
    message
  )
}
