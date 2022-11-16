use rand::Rng;

fn main() {
  let mut range = rand::thread_rng();

  let num: i32 = range.gen();

  println!("Random: {}", num);

  let mut n = rng.gen_range(1..99);
  println!("Value is {}", n);

  n = rng.gen_range(1..99);
  println!("Value is {}", n);
}
