#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

/*
线程安全的引用计数指针。‘Arc’代表 “Atomically Reference Counted/原子引用计数”。

类型 Arc 提供了一个 T 类型值的共享所有权，在堆中分配。
在Arc上调用clone会产生一个新的Arc实例，它指向与源Arc相同的堆上的分配值，同时增加一个引用计数。
当给定分配的最后一个Arc指针被销毁时，存储在该分配中的值（通常被称为 “内部值”）也会被丢弃。
*/

// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
fn main() {
  let numbers: Vec<_> = (0..100u32).collect();
  let shared_numbers = Arc::new(numbers); // TODO
  let mut joinhandles = Vec::new();

  for offset in 0..8 {
    let child_numbers = shared_numbers.clone(); // TODO
    joinhandles.push(thread::spawn(move || {
      let sum: u32 = child_numbers.iter().filter(|n| *n % 8 == offset).sum();
      println!("Sum of offset {} is {}", offset, sum);
    }));
  }
  for handle in joinhandles.into_iter() {
    handle.join().unwrap();
  }
}
