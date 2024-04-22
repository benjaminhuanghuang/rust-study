fn trim_me(input: &str) -> String {
  // TODO: Remove whitespace from both ends of a string!
  input.trim().to_string()
}

fn compose_me(input: &str) -> String {
  // TODO: Add " world!" to the string! There's multiple ways to do this!
  let mut res = input.to_string();
  res.push_str(" world!");
  res
}

fn replace_me(input: &str) -> String {
  // TODO: Replace "cars" in the string with "balloons"!
  let res = input.to_string();
  res.replace("cars", "balloons")
}
