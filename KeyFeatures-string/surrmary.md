# string Rules

- Always use String in structs

- Use String for function return values

- Use &str from function parameters

- If the return type of the function is derived from an argument and isn't mutated by the body, return &str. If you run
  into any trouble here, return String instead.
