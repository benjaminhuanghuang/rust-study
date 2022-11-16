#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
  Info,
  Warning,
  Error,
}

// "[ERROR]: the message"
pub fn log(level: LogLevel, message: &str) -> String {
  let label = format!("{:?}", level).to_uppercase();

  format!("[{}]: {}", label, message)
}
