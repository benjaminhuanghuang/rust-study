

{:.*} lets you specify the decimal precision of floating point numbers via the argument
```
format!("{:.*}", 2, 1.234567) // Returns "1.23
```

Name the parameters
```
 let introduction = format!(
    "My name is {surname}, {forename} {surname}", surname="Bond", forename="James"
  );
``` 

## padding 0
```
  let _0004_ = format!("{:0>4}", id);
```

## Hex
```
use std::fmt::{self, Display, Formatter, UpperHex};
// 在std中fmt::LowerHex和fmt::UpperHex可以用来输出16进制，分别对应的格式化符号是{:x}、{:X}。

```

