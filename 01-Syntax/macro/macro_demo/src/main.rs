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


fn main() {
    
  calculate!{
    eval 1+2
  }
  calculate!{
    eval (1+2) + (3+4)
  }
}
