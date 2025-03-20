# Arc

`Arc<T>` (Atomic Reference Counting) is a thread-safe version of `Rc<T>`.

Used for sharing data across multiple threads.

`Arc<T>` ensures safe memory access across threads using atomic operations.

```rs
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);

    let handles: Vec<_> = (0..3).map(|_| {
        let data = Arc::clone(&data); // Thread-safe cloning
        thread::spawn(move || {
            println!("{:?}", data);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
```
