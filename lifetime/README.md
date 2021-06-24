

## What is lifetime
- Rust allows one and only one owener of memory
- Rust allows multiple references
- Lifetimes enforce a piece of memoy is still valid fro a refernece

```
  let a;

  {
    let b = String::from("Hello");
    a = b;   // the ownership is transfered to a;
  }          // no memory clean

  println!("{}", a); // works fine
```

```
  let a;
  
  {
    let b = String::from("Hello");
    a = &b;   // ERROR! the memory that B owns is cleaned up at the end of the inner. a will be garbage.
  }          

  println!("{}", a); // works fine
```

## 
```
let r;
{
    let s1 = "rust";
    let s2 = "ecmascript";
    r = longer(s1, s2);             // ERROR, s1 or s2 was invalide
}
println!("{} is longer", r);


fn longer(s1: &str, s2: &str) -> &str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}


fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}
```



```
  fn get_in_ref<`a>(param_1: &`a i32) -> &`a i32 {
    param_1
  }
```