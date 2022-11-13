pub fn raindrops(n: u32) -> String {
  let is_factor = |f| n % f == 0;
  let mut sound = String::from("");
  if is_factor(3) {
    sound.push_str("Pling");
  }
  if is_factor(5) {
    sound.push_str("Plang");
  }
  if is_factor(7) {
    sound.push_str("Plong");
  }

  if sound.is_empty() {
    n.to_string()
  } else {
    sound
  }
}
