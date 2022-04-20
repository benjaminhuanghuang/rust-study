https://lotabout.me/2017/rust-error-handling/

错误：运行时发生的`不寻常的、 超出预期`的行为，这些问题只能通过修改程序来解决。例如内存不足。
异常 ：运行时发生的`不规则的、 意料之内` 的行为。例如尝试读取“读保护”的文件。



在 C 语言中，错误处理的机制是十分简陋的，例如 Linux 的系统调用如果出错，会将错误记录在一个全局变量 errno 中，errno 是一个整型值，操作系统事先约定好不同值代表不同含义。

到了 C++/Java/Python 语言则采用了异常处理机制，当函数错误时，可以抛出预定义或自定义的异常，语言本身提供了捕获这个异常/错误的语法（即 try ... catch ...）。 在某些情况下，异常处理所需要的额外性能开销是不可接受的


Rust 采用“返回错误信息” 的方式， 

Rust, errors can be classified into two major categories as shown in the table below.
- Recoverable : Errors which can be handled, return Resut<T, E>
- UnRecoverable: Errors which cannot be handled, panic!("Say something")

## Option
通常 一个函数执行某个任务，成功则返回执行结果失败则什么也不返回, Rust用 Option<T>
```
pub enum Option<T> {
    None,
    Some(T),
}
```
用match 来 处理option
```
match opt {
    Some(value) => println!("value = {}", value),
    None => println!("Got None"),
}
```
为了省去写match的麻烦，又提供了 unwarp
```
mpl<T> Option<T> {
    fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None =>
              panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```
通过调用 option.unwrap() 来获取 option 中包裹的值。即假设不可能出现 null ，但如果出了问题就退出。

如果知道可能出现 None 的情况，当出现时使用一个默认的值。rust 提供了函数 unwrap_or(default) 来方便书写。

如两个函数都返回 Option，我们想将一个函数的输出作为另一个函数的输入，此时可以使用 and_then 来减少手写 match 的次数。

# Result
如果一个函数可能发生多个，Option只能表示发生一个错误的情形。于是 Rust 提出了Result，用于包裹结果和错误
```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Error
在 Result 中，“错误”其实可以是任意类型。rust 定义了一个 trait: Error
```
  pub trait Error: Debug + Display {
    // snip
    fn source(&self) -> Option<&(dyn Error + 'static)> { None }
  }
```


