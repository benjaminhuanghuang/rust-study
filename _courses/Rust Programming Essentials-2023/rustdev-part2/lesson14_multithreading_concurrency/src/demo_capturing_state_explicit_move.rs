use std::thread;
use std::time::Duration;

pub fn do_it() {

    println!("\nIn demo_capturing_state_explicit_move::do_it()");

    let handle = do_some_work();
    handle.join().unwrap();

    println!("That's all, folks!");
}

fn do_some_work() -> thread::JoinHandle<()> {

    let v = vec![100, 101, 102, 103, 104, 105];

    // The compiler infers how to capture v, depending on how it's used in the closure. 
    // In this example, the compiler would guess a borrow is sufficient (because the for..in loop borrows vector v).
    // However, this would give a compiler error (because the closure might outlive the current scope and would have a dangling reference).
    // So we must use the "move" keyword to explicitly tell Rust to move captured state into the closure.
    let handle = thread::spawn(move || {
        for item in &v {
            println!("{:?} displaying {}", thread::current().id(), item);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // This wont't compile, because v was moved (explicitly) into the closure.
    // println!("{:?} displaying {:?}", thread::current().id(), v);

    // Return thread handle back to calling function. All owned local variables go out of scope.
    handle
}