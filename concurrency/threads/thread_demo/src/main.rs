use std::thread;
use thread::sleep;

fn main() {
  for i in 0..10 {
    let t = thread::spawn(move || {   // move i to thead.
      sleep(std::time::Duration::from_millis(100));
      println!("Thread {}", i)
    });
    t.join().;
  }

  println!("Main thread")
}
 