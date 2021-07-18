/*
Customize an error:

    1. impl std::fmt::Display的trait,并实现 fmt(...)方法

    2. impl std::fmt::Debug的trait，一般直接添加注解即可：#[derive(Debug)]

    3. impl std::error::Error的trait,并根据自身error级别是否覆盖std::error::Error中的source()方法
*/

use std::error::Error;
​
// 1. 自定义Error,通过#[derive(Debug)]注解实现std::fmt::Debug的trait
#[derive(Debug)]
struct CustomError {
    err: ChildError,
}
​
// 2. 实现Display的trait
impl std::fmt::Display for CustomError {
    // 一般情况下是固定写法
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "父类型错误~")
    }
}
​
// 3. 实现std::error::Error Trait,因为有子Error:ChildError,覆盖source()方法,返回Some(err)
impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.err)
    }
}
​
// 子Error
#[derive(Debug)]
struct ChildError;
​
// 实现Display
impl std::fmt::Display for ChildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "子类型错误~")
    }
}
​
// 实现Error的trait,因为没有子Error,不需要覆盖source()方法
impl std::error::Error for ChildError {}
​
// 构建一个Result的结果，返回自定义的error:CustomError
fn get_super_error() -> Result<(), CustomError> {
    Err(CustomError { err: ChildError })
}
​
fn main() {
    match get_super_error() {
        Err(e) => {
            println!("{}", e);
            println!("Caused by: {}", e.source().unwrap());
        }
        _ => println!("No error"),
    }
}