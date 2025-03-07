# async await

Rust 本身提供了 async/await 关键字，但不会自动执行 async 代码。它们的作用是 将异步代码转换为 Future，然后由一个 异步运行时（如 Tokio 或 async-std）来执行。

在编译过程中，Rust 的 async 代码会被转换为 状态机（state machine），而 await 关键字会被编译成 状态切换逻辑。

Rust 不会自动 poll 这个 Future，所以必须有某个组件（比如异步运行时）来不断调用它的 poll() 方法，使其推进状态。

运行时（如 Tokio 或 async-std）的作用是：

- 维护一个 任务队列，不断 poll 这些 Future，让它们推进执行。
- 处理 I/O 事件循环，在 Future 需要等待时，让出 CPU，等数据准备好后继续执行。

在 C++ 中，没有直接类似 Rust async/.await 的语言级支持，但可以使用 std::future、std::async、coroutines (C++20) 或 第三方库（如 Boost.Asio、Cppcoro）
来实现类似的异步编程。

```rs
async fn hello() {
    println!("Hello, world!");
}

fn main() {
    let future = hello(); // 这里不会执行，只是返回一个 Future
}
```

Rust 编译器会将 async fn 转换成：

```rs
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
```
