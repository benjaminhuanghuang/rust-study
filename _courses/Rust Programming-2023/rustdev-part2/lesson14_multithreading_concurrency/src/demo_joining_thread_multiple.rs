use std::thread;
use std::time::Duration;
use rand::Rng;

pub fn do_it() {

    println!("\nIn demo_joining_thread_multiple::do_it()");

    let mut handles: Vec<thread::JoinHandle::<_>> = vec![];

    for _ in 1..=5 {

        let handle = thread::spawn(|| {

            let mut rng = rand::thread_rng();
            let num = rng.gen_range(5..10);           

            println!("{:?} sleep for {} secs - starting", thread::current().id(), num);
            thread::sleep(Duration::from_secs(num));
            println!("{:?} sleep for {} secs - ended", thread::current().id(), num);
        });

        handles.push(handle);
    }

    for i in 100..=105 {
        println!("{:?} displaying {}", thread::current().id(), i);
        thread::sleep(Duration::from_millis(500));
    }

    println!("{:?} waiting for other threads to end", thread::current().id());

    for h in handles {
        h.join().unwrap();
    }

    println!("That's all, folks!");
}