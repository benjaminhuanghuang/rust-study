# Ch 1

## What is asynchronous programming

## When to use asynchronous programming

CPU-bound tasks:

- The computer must constantly work at them to make progress.
- An asynchronous coded version will run slower than a non-asynchronous version
- It has the added overhead of context switching between tasks

I/O-Bound Tasks

- The CPU spends time waiting for input and output operations to complete

Asynchronous can speed up I/O-Bound Tasks.

## How do the sync and await methods work in Rust

async: Denotes that the code following is asynchronous
await Method Call: Used on asynchronous functions; waits for the function state to change from pending to done

By default, Rust does not provide an asynchronous runtime.

We'll use Tokio's asynchronous runtime.

```rs
async fn hello_world() -> String {
  "Hello World".to_string ()
}

#[tokio::main]
async fn main() {
  let value = hello_world().await;
  println! ("{}", value);
}
```
