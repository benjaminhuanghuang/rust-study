# Rust Error Handling

- return a Reust<Data, Error>





## unwrap, expect
Get the **result**, and if there was an error, just panic and stop the program

unwrap and expecxe calls panic()

expect can send text message to panic
```
fn main() {
    let mut argv = env::args();
    let arg: String = argv.nth(1).unwrap(); // error 1
    let n: i32 = arg.parse().unwrap(); // error 2
    println!("{}", 2 * n);
}
```

Both `Option` and `Result` type have a method `unwrap()` defined on them.

## Option, Some, None, pattern matching, unwrap in Option
```
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

fn main_find() {
    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("No file extension found."),
        Some(i) => println!("File extension: {}", &file_name[i+1..]),
    }
}
```

Some is a variant or a value constructor for the Option type. 
You can think of it as a function with the type 
```
  fn<T>(value: T) -> Option<T>. 
```

Correspondingly, None is also a value constructor with the type 
```
  fn<T>() -> Option<T>.
```

unwrap in Option
```
enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None =>
              panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```


## Result, unwrap in Result

可能产生异常的函数的返回值都是 Result 类型

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}

type Option<T> = Result<T, ()>;
```


```
impl<T, E: ::std::fmt::Debug> Result<T, E> {
    fn unwrap(self) -> T {
        match self {
            Result::Ok(val) => val,
            Result::Err(err) =>
              panic!("called `Result::unwrap()` on an `Err` value: {:?}", err),
        }
    }
}
```