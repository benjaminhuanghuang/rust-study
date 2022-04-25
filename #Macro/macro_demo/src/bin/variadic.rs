macro_rules! multiply {
  // Edge case
  ( $last:expr ) => { $last };

  ( $head:expr, $($tail:expr), +) => {
      // define the return value as the $head multiplied with the multiplication of the $tail
      // Recursive call
      $head * multiply!($($tail),+)
  };
}

fn main() {
  // You can call multiply! with
  // as many parameters as you want
  let val = multiply!(2, 4, 8);
  println!("2*4*8 = {}", val)
}
