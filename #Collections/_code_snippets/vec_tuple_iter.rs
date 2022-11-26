pub enum Command {
  Uppercase,
  Trim,
  Append(usize),
}
pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
  let mut output: Vec<String> = vec![];

  for (string, command) in input.iter() {
    // TODO: Complete the function body. You can do it!
  }

  output
}
