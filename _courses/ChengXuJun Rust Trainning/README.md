# 程序君的 Rust 培训（1）
https://www.youtube.com/watch?v=ZVIlcsYaDZY

## 内存安全
- manually: C
- smart pointer: C++, Swift, ObjC
- GC: Java, C#
- Ownership: Rust
  
## 并发安全
- Single thread: JS
- Global thread lock: Python, Ruby
- CSP: Golang
- Ownership + Type System: Rust

## Rust 安全
- in one scope
  - only one ownership
  - drop when out of scope
  - can have multi immutable reference
  - Only can have one mutable reference(mutual exclusive)
  - lifetime of reference < lifetime of value
- Concurrency
  - Use Send / Sync