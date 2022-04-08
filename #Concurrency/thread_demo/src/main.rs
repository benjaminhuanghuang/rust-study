use std::thread;
use thread::sleep;

fn main() {
  let mut threads = vec![];


  for i in 0..10 {
    let t = thread::spawn(move || {   // move i to thead.
      sleep(std::time::Duration::from_millis(100));
      println!("Thread {}", i);
    });  
    
    threads.push(t);
  }
  
  // 阻止主线程退出
  for t in threads {
    t.join();  
  }
  println!("Main thread")
}
 