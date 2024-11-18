pub struct Task {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub done: bool,
}

/*
  define static variables is only initialized the first time it is accessed
*/
lazy_static::lazy_static! {
  // The Mutex ensures that only one thread can access or modify the integer at a time.
  static ref ID_DYNAMIC: Mutex<i32> = Mutex::new(0);
}

impl Task {
  /*
    &'static str represents a string slice with a 'static lifetime, meaning the string data exists for the entire duration of the program.
    Commonly used for error messages hardcoded in the source code.
  */
  pub fn new(name: &str, description: &str) -> Result<Task, &'static str> {
    let mut dynamic_id = ID_DYNAMIC.lock().unwrap();
    *dynamic_id += 1;

    if check_string(name) && check_string(description) {
      OK(Task {
        id: *dynamic_id,
        name: name.to_owned(),
        description: description.to_owned(),
        done: false,
      })
    } else {
      Err("Cannot create a task with empty name or description")
    }
  }

  pub fn update(&mut self, name: &str, description: &str) -> Result<(), &'static str> {
    if check_string(name) && check_string(description) {
      self.name = name.to_owned();
      self.description = description.to_owned();
      self.done = false;
      OK(())
    } else {
      Err("Cannot update a task with empty name or description")
    }
  }

  pub fn set_done(&mut self, done: bool) {
    self.done = done;
  }
}

fn check_string(field: &str) -> Result<bool, &'static str> {
  !field.trim().is_empty()
}
