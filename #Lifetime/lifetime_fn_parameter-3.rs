/*
  this function's return type contains a borrowed value,
  but the signature does not say whether it is borrowed from `x` or `y`
*/
//fn longest(x: &str, y: &str) -> &str {
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn main() {
  let string1 = String::from("long string is long");
  let result;
  // solution:
  //let string2 = String::from("xyz");
  {
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
  } //`string2` dropped here while still borrowed

  println!("The longest string is '{}'", result);
}
