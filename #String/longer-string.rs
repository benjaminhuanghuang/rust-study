fn longerStr<'a>(str1: &'a str, str2: &'a str) -> &'a str {
  if str1.len() > str2.len() {
    str1
  } else {
    str2
  }
}

fn main() {
  let str1 = "Hello";
  let str2 = "HelloWorld";

  let result = longerStr(str1, str2);
  println!("Longer string: {}", result);
}
