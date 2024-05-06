use std::thread;
use std::time::Duration;

pub fn do_it() {

    println!("\nIn demo_joining_thread_single::do_it()");

    let handle = thread::spawn(|| {
        
        println!("{:?} starting", thread::current().id());
        
        thread::sleep(Duration::from_secs(10));
        println!("{:?} ending", thread::current().id());
        
        // Uncomment the following code, to deliberately panic.
        // panic!("Deliberate panicking, dude!");
    });

    for i in 100..=105 {
        println!("{:?} displaying {}", thread::current().id(), i);
        thread::sleep(Duration::from_millis(500));
    }

    println!("{:?} waiting for other thread to end", thread::current().id());
    
    // Uncomment the following code, to unwrap the join() Result. This could panic!
    handle.join().unwrap();
    
    // Or uncomment the following code, to test the join() Result explicitly.
    // match handle.join() {
    //     Ok(_)  => println!("join() result is Ok"),
    //     Err(_) => println!("join() result is Err")
    // }

    println!("That's all, folks!");
}