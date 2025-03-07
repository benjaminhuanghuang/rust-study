/*
创建 5 个线程，每个线程都会 阻塞 2 秒。
如果有 1000 个任务, 线程数过多会导致 CPU 频繁上下文切换，影响性能
*/
use std::thread;
use std::time::Duration;

fn main() {
  let handles: Vec<_> = (0..5)
    .map(|i| {
      thread::spawn(move || {
        println!("Thread {} started", i);
        thread::sleep(Duration::from_secs(2)); // 模拟 I/O 操作
        println!("Thread {} finished", i);
      })
    })
    .collect();

  for handle in handles {
    handle.join().unwrap();
  }
}
