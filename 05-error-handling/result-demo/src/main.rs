fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
  if i == 42 {
      Ok(13.0)
  } else {
      Err(String::from("this is not the right number"))
  }
}

fn main() -> Result<(), String> {
  let v1 = match do_something_that_might_fail(2) {
    Ok(v) => v,
    Err(e) => {
      println!("found {}", e);
      return Err(e);
    },
  };
  println!("found {}", v1);

  // 看看我们节省了多少代码！
  let v = do_something_that_might_fail(42)?;
  println!("found {}", v);
  Ok(())
}
