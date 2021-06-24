



## Stack vs Heap
Stack
- Memory is auto recaptured by the program after variables go out of scope
- Is default in Rust
- Fixed size variables. Colleciotn can NOT be stack based ( Strings are a collection of u8)


Heap
- Flexibility
- Memory that can grow in size (Vector, HeashMap, String, etc)
- Memory can live beyond the scope.
- Heap memory is auto recaptured whtn the last OWNER of the memory goes out the scope.



## Ownership
Every piece of data in memory has an owner. 

There can only be a a **single owner** of memory at a time. (Borrow)

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

Borrow ownerhsip as a reference with the ampershand
```
  let heap_i8_2 = &heap_i8;            // Ownership MOVEed to heap_i8_2
```

Clone
```
  let heap_i8_2 = heap_i8.clone();     // Create a copy of the memory. Is relatively expensive.
```

## Move or borrow ownership in Procedure 
```
  int stack_f64: f64 = 1.;
  int heap_f64: Box<f64> = Box::new(2.);

  //--- Stack
  stack_procedure(stack_f64);                // stack_f64 is copied to param
  println("In main stack {}", stack_f64);    // stack_f64 is still avaliable 

  stack_procedure_mut(stack_f64);            // stack_f64 is copied to param
  println("In main stack {}", stack_f64);    // mutating parameter had no effect on stack_f64
  
  
  //---- Heap
  heap_procedure(heap_f64);                  // Ownership of memory associated with heap_f64 gets transferred to param
  println("In main stack {}", heap_f64);     // ERROR!

  heap_procedure(heap_f64.clone());          // heap_f64 is copied to the newly created memory used by param. It is expensive!
  println("In main stack {}", heap_f64);     

  heap_procedure_borrow(&heap_f6);          // procedure borrows the ownership and return back when procudure is done.
  println("In main stack {}", heap_f64);     


  fn stack_procedure(param: f64){
    println!("In stack_procedure with param {}", param);
  }

  fn stack_procedure_mut(mut param: f64){
    param += 9;
    println!("In stack_procedure with param {}", param);
  }

  fn heap_procedure(param: Box<f64>){
    println!("In heap_procedure with param {}", param);
  }

  fn heap_procedure_borrow(param: &Box<f64>){
    param += 9;
    println!("In heap_procedure with param {}", param);
  }
```


## String
string slice does not own a memory slot.

```
  let some_string: String = String::from("Hello");   // String are always on the heap
  let some_str: &str = "Partner";                    // &str is a pointer to either stack of heap.

  some_procedure(some_string, some_str);
  println!("{} {}", param_a, param_b);               // param_a is NOT avialable!

  
  some_procedure_borrow(&some_string, some_str);
  println!("{} {}", param_a, param_b);               // param_a is NOT avialable!


  fn some_procedure(param_a: String, param_b &str) {
    println!("{} {}", param_a, param_b);
  }

  fn some_procedure_borrow(param_a: &String, param_b &str) {
    println!("{} {}", param_a, param_b);
  }

```



```
  let var_a = String::from("foo");
  let var_b = &var_a;
  let var_c = &var_a;
  
  // Rust compiler will compaline, because var_b, var_c can not trust var_a
  var_a.push('a');  

  println!("{} {} {}", var_a, var_b, var_c);

  // This is ok
  var_a.push('a'); 

```


## String vector
```
  let var_a = String::from("foo");
  let var_b = String::from("bar");
  
  //let mass_data: Vec<&String> = vec![&var_a, &var_b];
  let mass_data: Vec<&str> = vec![&var_a, &var_b];

  println!("{}", heavy_cals(&mass_data));
  println!("{} {} {:?}", var_a, var_b, mass_data);

  fn heavy_calcs(param: &Vec<&String> -> i64 {

    10
  }

```

## Struct

```
  #[derive(Debug, Clone, Copy)]
  struct MyStruct{
    a: i32
  }

  let var_1 = MyStruct {a:9 , b:10}
  some_procedure(var_1)      // Move or copy

  fn some_procedure(param_a: MyStruct){

  }

  // I want to borrow and change the var_1
  fn some_procedure(param_a: &mut MyStruct){

  }
```