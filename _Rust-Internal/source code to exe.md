# Rust编译全揭秘-源码如何变身可执行程序

https://www.bilibili.com/video/BV1zVhN6oE9V

- 为什么同样一段代码，Debug 编译几乎瞬间完成，Release 编译却要等半天？
- 为什么加了一个宏（macro），代码报错的位置却指向了一堆"看不懂"的展开代码？
- 为什么改一行代码，有时候只需要重新编译几秒，有时候却要全量重新编译？
- 泛型函数 fn max<T: Ord>(a: T, b: T) -> T 明明只写了一份，为什么编译后的二进制文件里却"看起来"生成了好几份代码？

## rustc = frontend + LLVM backend

源码 → 词法分析 → 语法分析 → HIR → MIR → LLVM IR → 汇编 → 目标文件 → 可执行（右边还有箭头，后面可能还有内容）

前端 Front End: 词法/语法分析、宏展开、名称解析、类型检查、借用检查 —— 全部是 Rust 语言自己的逻辑, 把代码转换成 LLVM IR,剩下的
优化和生成机器指令由LLVM完成

后端 Back End（LLVM: 通用优化 + 翻译成目标平台机器码 —— 与具体语言无关，业界共享基础设施

- HIR（High-level Intermediate Representation，高层中间表示）
  来源：由 AST（抽象语法树）简化而来。
  特点：仍然很像源代码的结构，保留 for、if let、match 等写法的轮廓，但已经做了宏展开和名称解析，一些语法糖被拆开（比如 for 循环会变成 loop + match）。
  用途：类型检查和 trait 解析主要在这一层做。

- MIR（Mid-level Intermediate Representation，中层中间表示）
  来源：由 HIR 降低而来。
  特点：变成控制流图（CFG），由基本块（basic block）组成，没有复杂的嵌套表达式，每一步都是简单的赋值、调用、跳转。
  用途：
  借用检查（borrow checker），这是 Rust 内存安全的核心检查，就发生在 MIR 上。
  一些 Rust 自己的优化。
  泛型的单态化（monomorphization）也与这一层相关。

- LLVM IR
  来源：由 MIR 翻译而来，之后交给 LLVM。
  特点：一种与语言无关的低级、类似汇编的表示，带类型，使用无限个虚拟寄存器（SSA 形式）。C、C++、Swift 等语言的编译器也会生成它。
  用途：LLVM 在这一层做通用优化（内联、循环优化、常量折叠等），然后生成各平台的汇编和机器码。
  一个简单类比
- LLVM： 开源的编译器基础设施：一组可复用的库和工具，专门负责编译器的"后半段"，也就是优化代码并生成各平台的机器码。
