


## Rc
```
use std::rc::Rc;

fn bar(x: Rc<i32>) { }
fn baz(x: &i32) { }

fn foo() {
    let x = Rc::new(45);
    bar(x.clone());   // Increments the ref-count
    baz(&*x);         // Does not increment
    println!("{}", 100 - *x);
}  // Once this scope closes, all Rc pointers are gone, so ref-count == 0
   // and the memory will be deleted.
```