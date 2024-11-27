## Sample
Creat channel
```
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
      tx.send(1).unwrap();
  });

  println!("Got: {}", rx.recv().unwrap());
```


## Sample
```
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

```
`mpsc` stands for multiple producer, single consumer.

`tx` stands for transmitter, and `rx` stands for receiver.


## Multi producer/tx, need clone
```
  let (tx, rx) = mpsc::channel();

  for i in 0..THREAD_COUNT {
    thread_start(i, tx.clone());
  }

  for j in rx.iter().take(THREAD_COUNT) {
    println!("recv {}", j);
  }

  fn thread_start(d: usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
      println!("timer {}", d);
      thread::sleep(Duration::from_secs(d as u64));
      println!("send {}", d);
      tx.send(d).unwrap();
    });
  }
```
