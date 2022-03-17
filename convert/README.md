
Rust 使用 trait 解决类型之间的转换问题。最一般的转换会用到 From 和 into 两个 trait

```
use std::convert::From;


let my_str = "hello";
let my_string = String::from(my_str);
```

