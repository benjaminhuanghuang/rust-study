```rs
// Create
let mut num_map: HashMap<i32, i32> = HashMap::new();


// Iterate
for (index, &num) in nums.iter().enumerate() {

}

// Get
match general_map.get("test") {
  None => println!("it failed"),
  Some(result) => println!("Here is the result: {}", result)
}
```
