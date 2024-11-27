/*
'static means that any type implementing DisplayManager must either:
  Have no references that do not live for the entire duration of the program, or
  Contain only 'static references (data owned for the program's lifetime).

This is often used for traits in scenarios where the implementing type must not contain borrowed references with shorter lifetimes, such as in multithreaded contexts or for global variables.
*/
pub trait DisplayMessage: 'static {
  fn show(&self, message: String);
}
