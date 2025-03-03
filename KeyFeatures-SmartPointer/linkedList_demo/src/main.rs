use std::{ops::Deref, rc::Rc, sync::Arc, thread};
mod linkedlist_box;
mod linkedlist_rc;

fn main() {
  // Box can only have 1 owner
  let b1 = Box::new(1);
  let c = b1;
  //let c2 = b1; // ERROR

  // Box can only have 1 owner
  let b1 = Box::new(1);
  let h = Hello { v: b1 };

  // let h2 = Hello {
  //   v:b1                 // Error
  // };
  // Rc
  let r1 = Rc::new(10);
  let rcstruct = RcStruct { v: r1.clone() };
  let rcstruct2 = RcStruct { v: r1 };

  // Arc
  let arc1 = Arc::new(10);
  for _ in 0..10 {
    let arc = arc1.clone();
    thread::spawn(move || {
      println!("{}", arc);
    });
  }
}
struct Hello {
  v: Box<i32>,
}

struct RcStruct {
  v: Rc<i32>,
}

fn add(i: &i32) {}
