# Ch 2 The Basics

## Tokio's asynchronous runtime

```rs
#[tokio: :main]
```

- Copies code from main
- Starts an asynchronous runtime
- Pastes code into the runtime and calls it

```rs

#[tokio::main]
async fn main() {
  let value = hello_world().await;
  println! ("{}", value);
}

//-----Equals to

fn main() {
  tokio::runtime::Builder: :new_multi_thread()  // create the runtime
    .enable_all ()
    .build ()          // returns the runtime
    .unwrap ()
    .block_on( async { println! ("Hello World");});
}
```

The code from your main function is copied, pasted, and run somewhere.
Therefore, the original main function is never called.

## How to spawn a task

```rs
  let blocking_code_handle = tokio::task::spawn_blocking(blocking_call);

```

## How to spawn a asynchronous task

## Unit Test

```rs
#[tokio::test]
async fn test_add() {
  assert_eq! (add (1,1) .await, 2);
}
```
