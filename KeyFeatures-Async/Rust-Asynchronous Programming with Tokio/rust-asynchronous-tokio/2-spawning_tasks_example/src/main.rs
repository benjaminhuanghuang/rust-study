use std::{thread, time};

use tokio::time::{sleep, Duration};

fn blocking_call() -> String {
    thread::sleep(time::Duration::from_secs(5));

    "Finally done".to_string()
}

async fn async_call(id: i32) {
    sleep(Duration::from_secs(1)).await;
    println!("Async Call: ID {}", id);
}

#[tokio::main]
async fn main() {
    // run a blocking function (blocking_call) in a separate thread off the async runtime.
    let blocking_code_handle = tokio::task::spawn_blocking(blocking_call);

    let mut async_handles = Vec::new();
    for id in 0..10 { 
        // launches async_call(id) as an independent asynchronous task.
        async_handles.push(tokio::spawn(async_call(id)));
    }

    for handle in async_handles {
        handle.await.unwrap();
    }

    let result = blocking_code_handle.await.unwrap();
    println!("Blocking call: {}", result);
}
