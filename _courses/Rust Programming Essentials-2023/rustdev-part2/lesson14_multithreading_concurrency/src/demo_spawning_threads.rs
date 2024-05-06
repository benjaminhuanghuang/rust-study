use std::thread;
use std::time::Duration;

pub fn do_it() {

    println!("\nIn demo_spawning_threads::do_it()");

    thread::spawn(|| {
        for i in 1..=10 {
            println!("{:?} displaying {}", thread::current().id(), i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 100..=105 {
        println!("{:?} displaying {}", thread::current().id(), i);
        thread::sleep(Duration::from_secs(4));
    }

    println!("That's all, folks!");
}