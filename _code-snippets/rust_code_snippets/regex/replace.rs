use regex::Regex;

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
  let regex = Regex::new(target)?;
  Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
  let price = "$ 1,500";
  let regx = Regex::new(r"[^0-9.]").unwrap();

  println!("{}", regx.replace_all(price, ""));
}
