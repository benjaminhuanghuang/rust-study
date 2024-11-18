pub fn menu_logo() -> String {
  let logo = r#"
    _____        _       _    _    _   
  |_   _|__  __| |___  | |  (_)__| |_ 
    | |/ _ \/ _` / _ \ | |__| (_-<  _|
    |_|\___/\__,_\___/ |____|_/__/\__|
                                      
  "#
  .to_string();
  logo
}

pub fn menu_show() -> String {
  let options = r#" Add  to add a new task
  Remove   To remove a task
  Update   To update a task
  Display  To display all tasks
  Complete To mark a task as completed
  Save     To save the tasks
  Exit     To exit the program
  "#
  .to_string();
  options
}
