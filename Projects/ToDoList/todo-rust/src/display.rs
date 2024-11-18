pub trait DisplayManager: 'static {
  fn display(&self, todo_list: &ToDoList);
}
