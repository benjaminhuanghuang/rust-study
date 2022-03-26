## Strings
- UTF-8 Encoded

- Non-Null-Byte terminated

- Not collections of chars

String in memory

- Pointer to data
- Length
- Capacity

## The &str type
- A borrowed string slice
- Does not own string data
- Data not freed when dropped
- View/window into string data

&str memeory
- length
- data pointer

The Slice Type
  https://doc.rust-lang.org/stable/book/ch04-03-slices.html


## String literals
Embedded into the binary
String literals are all of type &str.


## multi-lines
```
r#"SELECT id, teacher_id, name, time 
    FROM course
    WHERE teacher_id = $1 and id = $2"#,
```