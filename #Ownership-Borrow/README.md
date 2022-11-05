







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