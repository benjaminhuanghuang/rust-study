use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn channel_demo1() {
  let (sender, receiver) = mpsc::channel();

  thread::spawn(move || {
    for i in 0..10 {
      thread::sleep(Duration::from_millis(100));
      sender.send(i).unwrap();
    }
  });

  while let Ok(v) = receiver.recv() {
    println!("{}", v)
  }
}



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
