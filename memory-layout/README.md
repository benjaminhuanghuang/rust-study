# Rust memory layout


## Reference
- Rust内存Layout（超清晰的讲解）
  - https://www.youtube.com/watch?v=rDoqT-a6UFg



Linux use ELF-64 exe format
- text segement : code, readonly
- data segement: local static, global
- BSS segment: Block started by symbol, contains uninitalized global variables
