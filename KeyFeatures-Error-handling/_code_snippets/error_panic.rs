fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
  // TODO: change this to return an appropriate error instead of panicking
  // when `parse()` returns an error.
  let x: i64 = s.parse().unwrap();
  PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
  let x = s.parse();
  match x {
    Err(e) => Err(ParsePosNonzeroError::from_parseint(e)),
    Ok(i) => PositiveNonzeroInteger::new(i).map_err(ParsePosNonzeroError::from_creation),
  }
}
