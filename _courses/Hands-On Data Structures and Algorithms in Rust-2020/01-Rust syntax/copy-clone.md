# Copy and Clone

Copies happen implicitly, for example as part of an assignment y = x. The behavior of Copy is not overloadable; it is always a simple bit-wise copy.

The Clone trait defines the ability to explicitly create a deep copy of an object T. When we call Clone for type T.

```rust
#derive[Clone, Copy]
pub struct Person {
  name: String;
  age: i32
}

let p2 = p.clone();

```
