pub enum Option<T> {
  Some(T),
  None,
}



fn main() {
  let a= divide(2,4 );
  let b= divide(2,0);

  println!("a={:?}, b={:?}", a, b);

  match a {
    Ok(v) => println!("val={}", v),
    _=>{}
  }


  if let Res::Thing(v) = a {
    println!("val ={}", v)
  }
}


fn divide(a: i32, b:i32) -> Result<i32, String> {
  if b==0{
    return Error("divide by 0".to_string());
  }

  Res::Thing(a/b);
}