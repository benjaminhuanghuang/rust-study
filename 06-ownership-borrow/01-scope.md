When the owner goes out of scope, the value will be dropped.

```
  fn main() {
    let say = String::from("Cat");
    
    print_out(say); // Transfore ownership to the function

    println!("Again: {}", say);  // Error!
  }


  fun print_out(to_print: String) {
    println!("{}", to_print);
  }
```