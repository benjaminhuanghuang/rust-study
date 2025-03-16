use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct HelloFuture;

impl Future for HelloFuture {
  type Output = ();

  fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
    println!("Hello, world!");
    Poll::Ready(())
  }
}

fn hello() -> impl Future<Output = ()> {
  HelloFuture
}

fn main() {
  let future = hello();
}
