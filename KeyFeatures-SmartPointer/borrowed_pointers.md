# Borrowed pointers borrowed references, or just references


use the & operator to create a borrowed reference and to indicate reference types, and * to dereference them. 
```
fn foo() {
    let x = &3;   // type: &i32
    let y = *x;   // 3, type: i32
    bar(x, *x);
    bar(&y, y);
}

fn bar(z: &i32, i: i32) {
    // ...
}
```

The & operator does not allocate memory (we can only create a borrowed reference to an existing value) and if a borrowed reference goes out of scope, no memory gets deleted.

you can have multiple borrowed references pointing to the same value


## mut
Mutable borrowed references are unique

If a mutable value is borrowed, it becomes immutable for the duration of the borrow.

Once the borrowed pointer goes out of scope, the value can be mutated again. This is in contrast to unique pointers, which once moved can never be used again. 


```
fn foo() {
    let mut x = 5;            // type: i32
    {
        let y = &mut x;       // type: &mut i32
        //x = 4;              // Error - x has been borrowed
        //println!("{}", x);  // Error - requires borrowing x
    }
    x = 4;                    // OK - y no longer exists
}
```
## Borrowing and lifetimes
The lifetime of the reference must be shorter than the lifetime of the referenced value.
```
fn foo() {
    let x = 5;
    let mut xr = &x;        // Ok - x and xr have the same lifetime
    {
        let y = 6;
        xr = &y             // Error - xr will outlive y
    }                       // y is released here
    println!("{?:}", xr);   // xr is used here so it outlives y. Try to comment out this line.
}                           // x and xr are released here
```
