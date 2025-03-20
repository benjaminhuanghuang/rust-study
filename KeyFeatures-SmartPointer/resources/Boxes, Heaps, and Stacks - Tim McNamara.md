# Boxes, Heaps, and Stacks - Tim McNamara - Rust Linz, September 2022

Author of Rust in Action

https://www.youtube.com/watch?v=DEE1GKMbtgw

```rs
  let n: Box<i32> = Box::new(123);
```

Box is a `pointer` type for heap allocation for a owned value of type T.

Box is a link between the stack and the heap as a pointer.

When box leaves scope, its contents are deleted/dropped. The trait that manages how this works is `Drop`

The context in this sentence:

- Pointer: memory address that compiler has information about the type it points to

  - memory address: a number that the CPU/hardware knows how to translate to a location on a physical chip (0 to 2^64 - 1)
  - Pointer in Rust

  ```rs
    &T
    &mut T
    *const T
    *mut T
    fn
    Box<T>
  ```

- Reference: a pointer that also has lifetime information.
- Heap

- Allocation: request memory from OS

- Owned value
