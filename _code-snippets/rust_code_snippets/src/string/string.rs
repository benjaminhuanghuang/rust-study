fn trim_me(input: &str) -> &str {
  // TODO: Remove whitespace from both ends of a string.
  input.trim_start()
}

fn compose_me(input: &str) -> String {
  // TODO: Add " world!" to the string! There are multiple ways to do this.h
  format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
  // TODO: Replace "cars" in the string with "balloons".
  input.replace("cars", "balloons")
}

fn string_slice(arg: &str) {
  println!("{arg}");
}

fn string(arg: String) {
  println!("{arg}");
}

fn string_or_slice() {
  string_slice("blue");

  string("red".to_string());

  string(String::from("hi"));

  string("rust is fun!".to_owned());

  string_slice("nice weather".into());

  string(format!("Interpolation {}", "Station"));

  // WARNING: This is byte indexing, not character indexing.
  // Character indexing can be done using `s.chars().nth(INDEX)`.
  string_slice(&String::from("abc")[0..1]);

  string_slice("  hello there ".trim());

  string("Happy Monday!".replace("Mon", "Tues"));

  string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
