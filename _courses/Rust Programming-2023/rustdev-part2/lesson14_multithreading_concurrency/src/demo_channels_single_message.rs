use std::thread;
use std::sync::mpsc;  

pub fn do_it() {

    println!("\nIn demo_channels_single_message::do_it()");

    // Create a channel to which we can send messages, and from which we can receive those messages. 
    let (tx, rx) = mpsc::channel();

    // Spawn a thread to send a message to the channel. 
    let handle = thread::spawn(move || {
        tx.send(String::from("Hei hei")).unwrap();
    });

    // On the main thread, receive the message from the channel.
    let received = rx.recv().unwrap();
    println!("Received: {}", received);

    handle.join().unwrap();
}