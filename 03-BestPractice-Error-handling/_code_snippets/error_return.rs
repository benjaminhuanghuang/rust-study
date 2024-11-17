#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
  Negative,
  Zero,
}

impl PositiveNonzeroInteger {
  fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
    // Hmm...? Why is this only returning an Ok value?
    if value > 0 {
      Ok(PositiveNonzeroInteger(value as u64))
    } else if value == 0 {
      Err(CreationError::Zero)
    } else {
      Err(CreationError::Negative)
    }
  }
}
