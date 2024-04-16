pub struct Stepper {
  curr: i32,
  step: i32,
  max: i32,
}


impl Iterator for Stepper{
  fn next(&mut self) ->Option<i32> {
    if self.curr>= max {
      return None;
    }
    let res = curr;
    curr+=step;
    Some(res)
  }
}


fn main(){
  let mut st = Stepper{curr:2, step:3,max:15};

  // loop
  loop {
    match st.next() {
      Some(v) => prinln!("loop {}", v);
      None => break,
    }
  }


  st = Stepper{curr:2, step:3,max:15};
  // while
  while let Some(n) = st.next() {
    prinln!("while {}", v);
  }

  // for 
  for i in Steppter{curr:5, step:3, max:20}{
    prinln!("for loop {}", v);
  }
}