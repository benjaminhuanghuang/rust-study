pub fn generate_nametag_text(name: String) -> Option<String> {
  if name.is_empty() {
    // Empty names aren't allowed.
    None
  } else {
    Some(format!("Hi! My name is {}", name))
  }
}

pub fn generate_nametag_text(name: String) -> Result<String, String> {
  if name.is_empty() {
    // Empty names aren't allowed.
    Err(String::from("`name` was empty; it must be nonempty."))
  } else {
    Ok(format!("Hi! My name is {}", name))
  }
}
