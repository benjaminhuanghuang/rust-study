use std::thread;
use std::time::Duration;

pub fn do_it() {

    println!("\nIn demo_capturing_state_implicit_move::do_it()");

    let handle = do_some_work();
    handle.join().unwrap();

    println!("That's all, folks!");
}

fn do_some_work() -> thread::JoinHandle<()> {

    let v = vec![100, 101, 102, 103, 104, 105];

    // The compiler infers how to capture v, depending on how it's used in the closure. 
    // In this example, the compiler decides a move is necessary (because the for..in loop requires ownership of vector v). 
    let handle = thread::spawn(|| {
        for item in v {
            println!("{:?} displaying {}", thread::current().id(), item);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // This wont't compile, because v was moved (implicitly) into the closure.
    // println!("{:?} displaying {:?}", thread::current().id(), v);

    // Return thread handle back to calling function. All owned local variables go out of scope.
    handle
}