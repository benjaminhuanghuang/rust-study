# Visualizing memory layout of Rust's data types
https://www.youtube.com/watch?v=rDoqT-a6UFg




## Linux use ELF-64 exe format
- text segment : code, readonly
- data segment: local static, global
- BSS segment: Block started by symbol, contains uninitialized global variables
