# what is async runtime

In Rust, an async runtime is a system that allows you to execute asynchronous tasks efficiently using Rust's async/await model.
Since Rust's standard library does not include a built-in async runtime, third-party libraries provide the necessary executors and reactors to handle async tasks.

## How It Works:

- Async/Await in Rust: Rust’s async functions return futures, which represent computations that may not be complete yet.
- Runtime Responsibility: The async runtime is responsible for polling these futures until they are completed.
- Concurrency & Scheduling: The runtime efficiently schedules and executes async tasks across available system resources (e.g., threads or event loops).
