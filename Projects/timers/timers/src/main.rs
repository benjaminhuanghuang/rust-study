use timers::autocorrelation;

pub fn main() {
  let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
  let result = autocorrelation(&data, 1);
  println!("Autocorrelation: {}", result);
}
