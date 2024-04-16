#[derive(Debug)]
ppub enum Res<T, E> {
  Thing(T),
  Error(E)
}


fn main() {
  let a= divide(2,4 );
  let b= divide(2,0);

  println!("a={:?}, b={:?}", a, b);

  match a {
    Res::Thing(v) => println!("val={}", v),
    _=>{}
  }


  if let Res::Thing(v) = a {
    println!("val ={}", v)
  }
}


fn divide(a: i32, b:i32) -> Res<i32, String> {
  if b==0{
    return Res::Error("divide by 0".to_string());
  }

  Res::Thing(a/b);
}