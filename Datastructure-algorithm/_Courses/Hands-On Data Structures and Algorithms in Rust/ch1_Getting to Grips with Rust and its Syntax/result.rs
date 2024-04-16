fn divide(a: i32, b: i32) -> Result<i32, String> {
  if b == 0 {
    return Err("Can not divide by zero".to_string());
  }
  Ok(a / b)
}

fn main() {
  let a = divide(4, 5);
  let b = divide(10, 0);

  println!("a = {:?}, b={:?}", a, b);

  match a {
    Ok(v) => println!("val = {}", v),
    _ => {}
  }

  if let Ok(v) = a {
    println!("val = {}", v)
  }
}
