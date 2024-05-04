use std::time::Duration;

pub fn do_it() {
	
    println!("\nIn demo_closures_capture_value::do_it()");

    capture_value_automatically();
    capture_value_forcibly();

    // Let's wait a bit before we quit... 
    std::thread::sleep(Duration::new(10, 0))
}

fn capture_value_automatically() {
    
    let message = String::from("hello");

    println!("Message initially:  {}", message);

    // consume_message moves 'message' into closure here.
    let consume_message = || {
        println!("Message in closure: {}", message);
        std::mem::drop(message);
    };
     
    // Can't use 'message' here, it's owned by the closure.
    // println!("{}", message);  // Nope!

    // We can call consume_message() once.
    consume_message();

    // But we can't call consume_message() again.
    // consume_message();        // Nope!
}

fn capture_value_forcibly() {
    
    let message = String::from("HELLO");

    println!("Start of method...");

    // We must forcibly move captured values into closure (otherwise it won't compile).
    std::thread::spawn(move || {
        println!("Message at start of closure: {}", message);
        std::thread::sleep(Duration::new(5, 0));
        println!("Message at end of closure:   {}", message);
    });
      
    println!("End of method...");
 }