# str, &str, String

It's all about "who owns the memory".

`String` contains a string in memory and owns the memory for it.

Use `String` for returning strings created within a function or (usually) when storing strings in a struct or enum.

If you have a `String` you can pass a reference to it to convert it to &str.

`&str` is just a reference to another string (slice) but does not own the memory for it.

Prefer `&str` in function arguments to accept string slices and make it clear the function will not mutate the string.

If you have a `&str` and want a new String you can clone it either by to_owned() or to_string() (they are effectively the same - use whichever makes your code clearer to read and consistent). These will copy the memory and make a new String.

```
let _val= "hello";                        // primitive string type immutable by default


let _val1= String::from("Hello, World!"); // Growable and mutable
```


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