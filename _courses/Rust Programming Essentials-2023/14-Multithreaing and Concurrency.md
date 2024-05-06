# Multithreading and Concurrency

https://learning.oreilly.com/videos/rust-programming-part/9780138316839/9780138316839-RPE1_04_14_00/

Rust provide functionality in the std::thread module

The API enables you to:
• Spawn a thread (i.e., create it), Get the thread ID
• Join a thread (i.e., wait for it to finish)
• Move data into a thread
• Communicate date between threads

## Spawn a thread

```rust
thread::spawn(|| {
  // do something

  thread::current().id();

  thread::sleep(Duration::from_secs(5));
});
```

## Join a thread

```rust
let handle: JoinHandle = thread::spawn(|| {
  // do something
});

// Wait for the spawned thread to finish
handle.join() {
  Ok(_) => println!("Thread finished"),
  Err(_) => println!("Thread panicked"),
}  
```


## Capture state from the main thread

Rust doesn't allow a thread closure to capture external state by borrowing. This could cause a dangling reference

A thread closure can only capture external state by moving
• The thread now owns the state
• The original function no longer owns it

```rust
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
```

## Communicate between threads using channels

A channel is a thread-safe pipeline between threads

One thread can send data in the channel, Another thread can read data from the channel
