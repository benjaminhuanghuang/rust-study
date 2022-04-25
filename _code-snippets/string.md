## Generate alphabetic
In order to be iterable, the range-type has to implement Step. char doesn't, so you wouldn't be able to use 'A'..'D' as an iterator.
```
   let alphabet: Vec<_> = (b'A' .. b'z' + 1) // Start as u8
        .map(|c| c as char)            // Convert all to chars
        .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
        .collect(); // Collect as Vec<char>

```

## Concatenating strings
```
fn main() {
  by_moving();
  by_cloning();
  by_mutating();


  // Use format!()
  let hello = "hello ";
  let world = "world!";
  let hello_world = format!("{}{}", hello, world);
  
  println!("{}", hello_world); // Prints "hello world!”

  // name your parameters!
  let introduction = format!(
    "My name is {surname}, {forename} {surname}", surname="Bond", forename="James"
  );
}

fn by_moving() {
  let hello = "hello ".to_string();
  let world = "world!";

  // Moving hello into a new variable
  let hello_world = hello + world;
  // Hello CANNOT be used anymore
  println!("{}", hello_world); // Prints "hello world!"
}

fn by_cloning() {
  let hello = "hello ".to_string();
  let world = "world!";

  // Creating a copy of hello and moving it into a new variable
  let hello_world = hello.clone() + world;
  // Hello can still be used
  println!("{}", hello_world); // Prints "hello world!"
}

fn by_mutating() {
  let mut hello = "hello ".to_string();
  let world = "world!";

  // hello gets modified in place
  hello.push_str(world);
  // hello is both usable and modifiable
  println!("{}", hello); // Prints "hello world!"
}
```