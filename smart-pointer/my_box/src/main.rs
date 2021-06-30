use std::ops::Deref;

fn main() {
    let i = 10;
    let r_i = &10;   //address on stack
    let j = *r_i;

    
    let b = Box::new(10);
    let b1 = *b;
    
    let myb = MyBox::new(10);
    let myb1 = *myb;   // *(mb.deref())

    {
      let bs = MyBox::new(String::from("Hello"));
      
      print_me(&bs);     // deref 
    }
}


fn print_me(s: &String) {
  println!("{}", *s);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(v: T)->Self {
    MyBox(v)
  }
}

// for * operation
impl<T> Deref for MyBox<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

// when out of scope
impl<T> Drop for MyBox<T> {
  fn drop(&mut self) {
    println!("Drop MyBox");
  }
}