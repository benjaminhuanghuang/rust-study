
src/main.rs和src/lib.rs都是crate的根，也就是crate引用、rustc编译的入口

一个package中可以有
- 多个bin类型的crate
- 0个或1个lib类型的crate


如果有多个bin，一个src/main.rs就不行了，就得放到 src/bin 下面，每个crate一个文件，换句话说，每个文件都是一个不同的crate


一般来说，一个文件都会被视为一个mod，而且mod可以嵌套定义。嵌套定义的mod既可以写在同一个文件里，也可以通过文件夹的形式来实现。

