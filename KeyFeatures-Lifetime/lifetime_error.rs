/*
The Rust compiler needs to know how to check whether supplied references are
valid, so that it can let the programmer know if a reference is at risk of
going out of scope before it is used.

Error: missing lifetime specifier
this function's return type contains a borrowed value, but there is no value for it to be borrowed from `x` or `y`
*/
fn longest(x: &str, y: &str) -> &str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

/*
Fixed
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

/*

 string2 is declared inside the inner block and gets dropped at the end of the block.
 However, result is a reference to either string1 or string2, meaning it might
 reference a dangling value if string2 is chosen.
*/
fn test() {
  // TODO: Fix the compiler error by moving one line.

  let string1 = String::from("long string is long");
  let result;
  {
    let string2 = String::from("xyz");
    result = longest(&string1, &string2);
  }
  println!("The longest string is '{result}'");
}

/*
Fixed
*/
fn test() {
  let string1 = String::from("long string is long");
  let string2 = String::from("xyz");

  let result = longest(&string1, &string2);

  println!("The longest string is '{result}'");
}

/*
lifetimes ('a) ensure that references stored in a struct remain valid for as long as the struct exists.
Rust does not allow structs to store references without explicit lifetime annotations.
*/
struct Book<'a> {
  author: &'a str,
  title: &'a str,
}

/*
title and author are created inside the function.
When the function ends, they are dropped.
The returned Book struct contains invalid references → Rust prevents this!
*/
fn create_book() -> Book<'_> {
  let title = String::from("Temporary Book");
  let author = String::from("Unknown");
  Book {
    author: &author,
    title: &title,
  } // ❌ ERROR: `title` and `author` get dropped!
}
