pub enum Command {
  Uppercase,
  Trim,
  Append(usize),
}

mod my_module {
  use super::Command;

  // TODO: Complete the function signature!
  pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
    // TODO: Complete the output declaration!
    let mut output: Vec<String> = vec![];
    for (string, command) in input.iter() {
      // TODO: Complete the function body. You can do it!
      match command {
        Command::Uppercase => output.push(to_uppercase(&string)),
        Command::Trim => output.push(trim(&string)),
        // I get a mismatched type error here
        Command::Append(n) => output.push(append_bar(&string, n)),
      }
    }
    output
  }

  fn to_uppercase(string: &str) -> String {
    string.to_uppercase()
  }

  fn trim(string: &str) -> String {
    string.trim().to_string()
  }

  fn append_bar(string: &str, count: usize) -> String {
    let bar_list: Vec<&str> = Vec::new();
    let mut result = String::from(string);
    for _ in 0..count {
      result += "bar";
    }
    result
  }
}

#[cfg(test)]
mod tests {
  // TODO: What do we have to import to have `transformer` in scope?
  use super::Command;
  use my_module::transformer;

  #[test]
  fn it_works() {
    let output = transformer(vec![
      ("hello".into(), Command::Uppercase),
      (" all roads lead to rome! ".into(), Command::Trim),
      ("foo".into(), Command::Append(1)),
      ("bar".into(), Command::Append(5)),
    ]);
    assert_eq!(output[0], "HELLO");
    assert_eq!(output[1], "all roads lead to rome!");
    assert_eq!(output[2], "foobar");
    assert_eq!(output[3], "barbarbarbarbarbar");
  }
}
