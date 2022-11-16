// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
  start + Duration::from_secs(11111)
}
