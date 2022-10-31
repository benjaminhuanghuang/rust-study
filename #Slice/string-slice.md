
## String的切片和普通的切片有些不同。

String的切片类型是str，而非[String]，String切片的引用是&str而非&[String]。

Rust为了保证字符串总是有效的Unicode字符，它不允许用户直接修改字符串中的字符，所以也无法通过切片引用来修改源字符串，除非那是ASCII字符(ASCII字符总是有效的unicode字符)。

Rust只为&str提供了两个转换ASCII大小写的方法来修改源字符串，除此之外，没有为字符串切片类型提供任何其他原地修改字符串的方法。

```
fn main(){
  let mut s = String::from("HELLO");
  let ss = &mut s[..];

  // make_ascii_lowercase()
  // make_ascii_uppercase()
  ss.make_ascii_lowercase();
  println!("{}", s);  // hello
}
```