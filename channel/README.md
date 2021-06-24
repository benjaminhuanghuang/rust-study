
## Create a channel
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