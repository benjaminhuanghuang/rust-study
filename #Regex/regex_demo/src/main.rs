use regex::Regex;

fn main() {
  let price = "$ 1,500";
  let regx = Regex::new(r"[^0-9.]").unwrap();

  println!("{}", regx.replace_all(price, ""));
}
