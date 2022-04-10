fn main() {
    let text = "hello";

    println!("{}", first_line(text));
}


pub fn first_line(string: &str) -> &str {
  string.lines().next().unwrap();
} 