
## Clone
```
  fn main() {
    let say = String::from("Cat");
    
    print_out(say.clone()); // create copy in heap and stack

    println!("Again: {}", say);  // Error!
  }


  fun print_out(to_print: String) {
    println!("{}", to_print);
  }
```

## Borrow
Can have any number of immutable references

References must always be valid

```
  fn main() {
    let say = String::from("Cat");
    
    print_out(&say); //  pass reference

    println!("Again: {}", say);  
  }


  fun print_out(to_print: &String) {   // Pass the reference
    println!("{}", to_print);
  }
```

## Mutable borrowing
Can have only one mutable references

```
  fn main() {
    let mut my_vec = vec![1, 2,3 ];
    
    add_to_vec(&mut my_vec); // &mut Pass the mutable reference
    
    println!("{:?}", my_vec);  
  }


  fun add_to_vec(vec: &mut Vec<i32>) {   // &mut Pass the mutable reference
    vec.push(4);
  }
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