
```
match general_map.get("test") {
  None => println!("it failed"),
  Some(result) => println!("Here is the result: {}", result)
}
```