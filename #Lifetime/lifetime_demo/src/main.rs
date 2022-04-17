/*
  will be move to other scope by default
*/
struct Hello {
  v: i32,
}

fn consume(h: Hello) {
  println!("{}", h.v)
}


fn main() {
  let h = Hello{ v:30};
  println!("{}", h.v);

  consume(h);
}
