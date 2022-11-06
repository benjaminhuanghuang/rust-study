

```
// 含义：foo的生命周期不能超出 'a 和 'b 中任一个的周期

fn f<'a, 'b>(v1: &'a T1, v2: &'b T2) -> &'a T3 {
  ...
}
```
传入两个指针 v1 v2，返回一个指针。
v1 的 lifetime 和 v2 不同，但是和最终返回的指针相同。


实体A持有一个指向实体B的引用，则A能够访问期间B必须存活

```
//正确： 一个参数，编译器能推断生命周期，不用注明 
fn longest1(x: &str) -> &str {
    x
}
 
//错误： 二个参数，这让编译器犯迷糊。即使没使用，也不行,编译不过去
fn longest2(x: &str,y:&str) -> &str {
    print!("{}",y);
    x
}
```
如果有多个参数，你就得告诉编译器返回的引用是跟着哪个参数的生命周期。
```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```




```
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
  *i + *j
}
```

<'a, 'b> declares two lifetime variables, 'a and 'b, within the scope of add_with_lifetimes(). These are normally spoken as lifetime a and lifetime b.

i: &'a i32 binds lifetime variable 'a to the lifetime of i. The syntax reads as “parameter i is a reference to an i32 with lifetime a.”

j: &'b i32 binds the lifetime variable 'b to the lifetime of j. The syntax reads as “parameter j is a reference to an i32 with lifetime b.”