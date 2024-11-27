


- 如果一个对象是 static， 说明对象存储在 bss/data/text section中， 内存地址固定
  - constants / static variables
  - string literals
  - functions

- 如果在trait bound 中使用， 
  - 类型不包括任何非静态的引用（non-static reference）
- 有ownership的数据的lifetime bound 是 'static, 引用数据不是'static
- 
