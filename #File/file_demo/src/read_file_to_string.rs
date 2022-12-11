use std::fs::*;


fn print_wanted_line_from_file(path: &str, wanted: &str) {
  let content:String = fs::read_to_string(file_path).expect("Failed to read");
  // fs::read_to_string call File.read_to_string(&mut string)
  for line in content.lines() {
    if line.contains(wanted) { 
      println!("{}", line);
    }
  }
}


fn main() {
  let file_path = "test.txt"
  let wanted = "a";
  
  print_wanted_line_from_file(file_path, wanted);
}
