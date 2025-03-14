# What is difference between &str and &String

## &str

- A reference to a string slice, which is a view into a part of a string.
- Typically used for static strings (e.g., "Hello") or substrings of a larger string.
- It is a borrowed, immutable reference.

```rs
fn print_str(s: &str) {
    println!("{}", s);
}

let greeting = "Hello, world!"; // &str
print_str(greeting);
```

## &String

- A reference to a String, which is an owned, growable string stored on the `heap`.
- &String is simply a borrowed reference to a String type.

```rs
fn print_string(s: &String) {
    println!("{}", s);
}

let owned_string = String::from("Hello, world!"); // String
print_string(&owned_string);
```

## Difference

| Feature     | &str                                       | &String                                                  |
| ----------- | ------------------------------------------ | -------------------------------------------------------- |
| Size        | Fat pointer, containing a reference to the | Fat pointer , It is a fat pointer containing a reference |
|             | data and the length of the slice.          | to the data and the length of the slice.                 |
| Ownership   | Does not own data                          | Borrowed from a String                                   |
| Use case    | Static strings, substrings                 | When a function needs a String                           |
| Performance | Lightweight, no heap allocation            | Similar to &str                                          |

&str is more general and preferred in most cases because it can work with both String and string slices.

&String is less common and used when you need to explicitly borrow a String.

&str Can refer to either stack or heap data. &String Refers to a String on the heap.
