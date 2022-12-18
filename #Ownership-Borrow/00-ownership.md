

Rust stores data in two differently strudture parts of memory


## Stack vs Heap
Stack
- Last In, first Out
- Memory is auto recaptured by the program after variables go out of scope
- Is default in Rust
- Fixed size variables. Colleciotn can NOT be stack based ( Strings are a collection of u8)
```
  // fixed size at compile time
  let a: i32 = 5;
```

Heap
- Flexibility
- Memory that can grow in size (Vector, HeashMap, String, etc)
- Memory can live beyond the scope.
- Heap memory is auto recaptured whtn the last OWNER of the memory goes out the scope.


```
  // vector object stored on stack with a pointer to heap 
  let mut vec1 = vec![1, 2, 3];

  // change the size
  vec1.push(4) 
```


## Ownership
- Every piece of data in memory has an owner. 
- There can only be a **single owner** of memory at a time. (Borrow)
- Fix Dangling pointer
- Fix Double-free
- Fix Memeory leak

```
  let stack_i8: i8 = 10;
  let stack_i8_2: i8 = stack_i8;     // copy memory on Stack. stack_i8 and stack_i8_2 own different memory.
  printlin!("{}", stack_i8);
  printlin!("{}", stack_i8_2);    
  
  let heap_i8: Box<i8> = Box::new(30);
  let heap_i8_2 = heap_i8;            // Ownership MOVEed to heap_i8_2
  printlin!("{}", heap_i8);           // ERROR!
  printlin!("{}", heap_i8_2);    
```

### Move
```
  let x = vec![1, 2, 3];
  let y = x;
  // x is move to y


  let heap_i8_2 = &heap_i8;            // Ownership MOVEed to heap_i8_2


  let s = String::from("abc");
  takes_ownership(s);              // give ownership to function


  fn give_ownership() -> String {
    "give".to_string()
  }
  let str = give_ownership();
```

## Borrow: immutable or mutable

Borrow ownership as a reference with the ampershand
```
  let x = vec![1, 2, 3];
  println!("{:?}", x);    // borrow
```

mutable reference
```
fn change_str(str: &mut String) {
  str.push_str(", hi");
}

let mut s = String::from("foo");
change_str(&mut str);
println("{}", s);
```

## Clone, deep copy
```
  let heap_i8_2 = heap_i8.clone();     // Create a copy of the memory. Is relatively expensive.
```

