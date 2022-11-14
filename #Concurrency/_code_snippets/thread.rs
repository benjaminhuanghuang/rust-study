use std::thread;


fn main() {
  let outer_scope = 123;

  let join_handler = thread::spawn(move||{
    outer_scope *2
  })

  let result = join_handler.join();
  
  match result {
    Ok(value) => println!("{}", value);
    Err(err) => println!("{}", err);
}