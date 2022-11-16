## Advanced
https://learn.microsoft.com/en-us/training/modules/rust-introduction/2-rust-overview

Rust also offers the following advantages that make it a great fit for a wide range of applications:

- Type safe: The compiler assures that no operation will be applied to a variable of a wrong type.

- Memory safe: Rust pointers (known as references) always refer to valid memory.

- Data race free: Rust's borrow checker guarantees thread-safety by ensuring that multiple parts of a program can't mutate the same value at the same time.

- Zero-cost abstractions: Rust allows the use of high-level concepts, like iteration, interfaces, and functional programming, with minimal to no performance costs. The abstractions perform as well, as if you wrote the underlying code by hand.

- Minimal runtime: Rust has a very minimal and optional runtime. The language also has no garbage collector to manage memory efficiently. In this way Rust is most similar to languages like C and C++.

- Targets bare metal: Rust can target embedded and "bare metal" programming, making it suitable to write an operating system kernel or device drivers.


## Resource: memory, socket, file problem
  - Use after free
  - Double free
  - Memory leak


## Rust programs are free from

- Dangling pointer — Live references to data that has become invalid over the course of the program 

- Data races — The inability to determine how a program will behave from run to run because external factors change 

- Buffer overflow — An attempt to access the 12th element of an array with only 6 elements

- Iterator invalidation — An issue caused by something that is iterated over after being altered midway through 