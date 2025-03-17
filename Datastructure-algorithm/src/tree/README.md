
https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370

- Rust 速成课/二叉树
http://accu.cc/content/rust_crash/binary_tree/

Rc 是引用计数指针，通常用来处理单个值拥有多个所有者的情况. 每当这个值拥有一个所有者, 那它的引用计数加 1. 当它的引用计数变为 0 时, 则内存被回收. 通过 clone 的方式可以被多个变量拥有对应的引用所有权，但是存储于 Rc 指针中的值是不可变的。

如果想改变Rc中的值，需要使用内部可变的 RefCell 指针。


- Rc,RefCell or Box
https://github.com/pretzelhammer/rust-blog/blob/master/posts/learning-rust-in-2020.md#leetcode

- 为什么leetCode中二叉树的rust实现使用的Rc、RefCell，而不使用Box
https://rustcc.cn/article?id=2402b3fe-0180-4126-a8e3-90cfd837c476

官方书籍的相关描述： Box 允许在编译时执行不可变或可变借用检查；
Rc仅允许在编译时执行不可变借用检查；
RefCell 允许在运行时执行不可变或可变借用检查。
