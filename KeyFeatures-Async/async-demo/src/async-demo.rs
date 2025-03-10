/*
只用一个线程，但可以同时执行多个任务（异步 I/O，不会阻塞主线程）。
更高效：适合高并发（如 10K 任务）。

*/
use tokio::time::sleep;
use std::time::Duration;

async fn async_task(i: u32) {
    println!("Task {} started", i);
    sleep(Duration::from_secs(2)).await; // 异步等待，不阻塞线程
    println!("Task {} finished", i);
}

#[tokio::main]
async fn main() {
    let mut tasks = vec![];
    for i in 0..5 {
        tasks.push(tokio::spawn(async_task(i))); // 不创建新线程
    }
    for task in tasks {
        task.await.unwrap();
    }
}
