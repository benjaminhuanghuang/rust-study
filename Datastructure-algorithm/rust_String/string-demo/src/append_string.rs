let mut sound = String::from("");

sound.push_str("Pling"); 

sounds += "Plang";

// stringbarbarbar
fn append_bar(string: &str, count: &usize) -> String {
  string.to_string() +&"bar".repeat(*count)  
}

fn append_bar(string: &str, count: &usize) -> String {
  let mut result = String::from(string);
  for _ in 0..*count {
      result += "bar";
  }
  result
}
