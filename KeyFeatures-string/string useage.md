# Sting in Rust

## Parameter and return value

```rs
pub fn new(name: &str, description: &str) -> Result<Task, &'static str> {
  if name.is_empty() {
    return Err("Task name cannot be empty");
  }
}
```

To the parameter, the use of &str instead of String suggests the function doesn’t own the string data; it merely borrows it.

To the Result, Err(&'static str): Indicates an error, returning a static string slice describing the problem.
A string slice with a 'static lifetime, meaning the string data exists for the entire duration of the program. 
Commonly used for error messages hardcoded in the source code.