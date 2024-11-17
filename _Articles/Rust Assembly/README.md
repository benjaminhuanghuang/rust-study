# Rust基础语句汇编分析
https://rustmagazine.github.io/rust_magazine_2021/chapter_12/simple-rust-in-assembly.html


## no_mangle 是什么
Rust 在编译时会修饰实体名，保证实体名字必须唯一。
比如函数名mangled在经过名字修饰后，会变为 _ZN8assembly7mangled17hdae1ffd2c2da5f53E，
具体的规则可以查看[RFC](https://rust-lang.github.io/rfcs/2603-rust-symbol-name-mangling-v0.html)。

为了可以找到编译后的函数，需要为函数加上#[no_mangle]这一属性，保证函数保留原本的名称。

## start 是什么
Rust在执行时，函数调用过程为 _start -> main -> our_main

experiment:
```
# 首先读取ELF头，得到入口点地址为0x6470

# 查看Objdump的结果：
```
