
## Terms
- workspace: 项目复杂时，管理多个package

- package: cargo new 命令会创建一个新项目，也是一个package, 管理crate

- crate: 模块的集合，编译单位，有lib和bin两种，即供别人调用，或者是一个可执行文件

- module: 用于在crate内组织代码. 代码多了可以对代码以mod（文件/文件夹）为单位进行拆分
一般来说，一个文件都会被视为一个mod，而且mod可以嵌套定义。嵌套定义的mod既可以写在同一个文件里，也可以通过文件夹的形式来实现。



## Folder structure for Rust project

Cargo.toml与Cargo.lock存储在项目的根目录中。

源代码进入src目录。

默认库文件是src/lib.rs。

默认的可执行文件是src/main.rs。

其他可执行文件可以放入src/bin/*.rs。

集成测试进入tests目录（Unit test 放入要测试的每个文件中）。

示例可执行文件放在examples目录中。

基准测试进入benches目录。


