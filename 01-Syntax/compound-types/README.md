# Compound data types

## Array
- Fixed length, length known at compile time
- Heterogeneous 单一类型


## Tuples
- Fixed length, length known at compile time
- Homogenous
- empty tuble is called unit


## Struct
```
// Classic struct with named fields
struct Student { name: String, level: u8, remote: bool }

// Tuple struct with data types only
struct Grades(char, char, char, char, f32);

// Unit structs are most commonly used as markers
struct Unit;
```


## Enum

Enum vs Struct
- in enum each variant can have a different type
- All variants sorted under the custom enum type / 节省空间

https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html

