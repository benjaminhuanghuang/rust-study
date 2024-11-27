#![allow(dead_code)]
struct Hello {
  v: i32,
}
/*
  will be move to other scope by default
*/
fn consume(h: Hello) {
  println!("{}", h.v)
}

/*
  will be move to other scope by default
*/
fn consume_reference(h: &Hello) {
  println!("{}", h.v)
}

fn main() {
  let h = Hello{ v:30};
  

  println!("{}", h.v);

  // consume(h);

  consume_reference(&h);
  consume_reference(&h);
}
