# What is difference between &str and &String

## &str

### What it is &str

- A reference to a string slice, which is a view into a part of a string.
- Typically used for static strings (e.g., "Hello") or substrings of a larger string.
- It is a borrowed, immutable reference.

### Characteristics

- Size: It is a fat pointer containing a reference to the data and the length of the slice.
- Ownership: Does not own the data it references. The data must exist elsewhere.
- Performance: Lightweight because it does not involve heap allocation or ownership checks.

```rs
fn print_str(s: &str) {
    println!("{}", s);
}

let greeting = "Hello, world!"; // &str
print_str(greeting);
```

## &String

### What it is &String

- A reference to a String, which is an owned, growable string stored on the heap.
- &String is simply a borrowed reference to a String type.

### Characteristics

- Size: Like &str, it is also a fat pointer.
- Ownership: Borrowed from a String. The String must exist while the &String reference is used.
- Use case: Often used when a function needs to work with an owned String but doesn't need to take ownership of it.

```rs
fn print_string(s: &String) {
    println!("{}", s);
}

let owned_string = String::from("Hello, world!"); // String
print_string(&owned_string);
```

## Difference

&str is more general and preferred in most cases because it can work with both String and string slices.

&String is less common and used when you need to explicitly borrow a String.

&str Can refer to either stack or heap data. &String Refers to a String on the heap.
