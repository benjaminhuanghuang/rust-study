#![allow(unused)]
macro_rules! calculate {
  // $e 为expr类型, match 1 + 2
  (eval $e:expr) => {{
    {
      let val: usize = $e;
      println!("{} = {}", stringify!{$e}, val);
    }
  }};
}

macro_rules! gcd {
  ($a: expr, $b: expr) => {
    let mut m = $b;

    let mut n = $a;

    while m != 0 {
      if m < n {
        let t = m;
        m = n;
        n = t;
      }
      m = m % n;
    }
    n
  };
}

fn main() {
  calculate! {
    eval 1+2
  }
  calculate! {
    eval (1+2) + (3+4)
  }

  println!("{}", gcd!(14, 15))
}
