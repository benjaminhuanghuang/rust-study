use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const THREAD_COUNT: usize = 20;

fn thread_start(d: usize, tx: mpsc::Sender<usize>) {
  thread::spawn(move || {
    println!("timer {}", d);
    thread::sleep(Duration::from_secs(d as u64));
    println!("send {}", d);
    tx.send(d).unwrap();
  });
}

fn main() {
  let (tx, rx) = mpsc::channel();

  for i in 0..THREAD_COUNT {
    thread_start(i, tx.clone());
  }

  for j in rx.iter().take(THREAD_COUNT) {
    println!("recv {}", j);
  }
}
