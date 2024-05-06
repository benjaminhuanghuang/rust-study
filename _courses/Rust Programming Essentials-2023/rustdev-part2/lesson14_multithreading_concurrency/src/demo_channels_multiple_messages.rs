use std::thread;
use std::time::Duration;
use std::sync::mpsc;  

pub fn do_it() {

    println!("\nIn demo_channels_multiple_messages::do_it()");

    // Create a channel to which we can send messages, and from which we can receive those messages. 
    let (tx, rx) = mpsc::channel();

    // Spawn a thread to send some messages to the channel. 
    let handle = thread::spawn(move || {
        for i in 1..=10 {
            let s = std::format!("Message {}", i);
            tx.send(s).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // On the main thread, receive the messages from the channel.
    for received in rx {
        println!("Received: {}", received);
    }

    handle.join().unwrap();
}
