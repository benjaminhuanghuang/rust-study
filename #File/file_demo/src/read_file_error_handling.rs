use std::fs::*;


// error handling
fn read_file(path: &str) -> Result<Vec<String>, std::io::Error> {
  
  let content = fs::read_to_string(file_path)?;
  
  let mut lines = Vec::new();
  for line in content.lines() {
    lines.pus(line.to_string());
  }

  Ok(lines)
}


fn read_file_2(path: &str) -> Result<Vec<String>, std::io::Error> {
  
  let content = fs::read_to_string(file_path)?;
  
  let mut lines = content.lines().map(|line| line.to_string()).collect();

  Ok(lines)
}


fn get_words(path: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
  let content = fs::read_to_string(file_path)?;
  
  let words = content.lines().map(|line| {
    line.split_whitespace().map(|word| word.to_string()).collect()
  }).collect();

  Ok(lines)
}



fn main() {
  let file_path = "test.txt"

  let lines = read_file(&file_path).expect(&format!("unable to read file <{}>", file_path));

  println!("{:#?}", lines);


  let words = get_words(&file_path).expect(&format!("unable to read file <{}>", file_path));

  println!("{:#?}", words);
}
