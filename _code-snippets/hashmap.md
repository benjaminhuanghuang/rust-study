# HashMap

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

## Modify

```rs

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: Declare the hash map.
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket.
    // Add more bananas to the basket
    *basket.entry(String::from("banana")).or_insert(0) += 2;


    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("orange"), 2);

    basket
}
```
