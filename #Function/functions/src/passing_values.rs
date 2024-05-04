pub fn do_it() {
  println!("\nIn demo passing_values::do_it()");

  let n = 42;
  let s = String::from("Hello");
  some_func(n, s);

  println!("n = {}", n);
  //println!("s = {}", s); // can not use s here
}

fn some_func(n: i32, s: String) {
  println!("In some_func, n = {}", n);
  println!("In some_func, s = {}", s);
}
