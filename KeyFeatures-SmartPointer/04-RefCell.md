# RefCell

Allows mutable borrowing inside immutable structures (interior mutability).

`RefCell<T>` allows mutable borrowing at runtime, even if the variable is immutable.

It enables interior mutability, which `Box<T>` and `Rc<T>` do not allow.

`RefCell<T>` let us have `many immutable` borrows or `one mutable` borrow.

in Rust, you generally cannot mutate data through an immutable reference (Rust enforces strict borrowing rules to ensure memory safety).
However, RefCell allows you to bypass this restriction in a controlled way.

Unlike Rust's default compile-time borrow checking, RefCell uses runtime borrow checking.
It ensures that at runtime, the borrow rules are followed.

This is useful when you need to have more flexible borrowing in situations like mutating values inside data structures (e.g., trees, linked lists),
but still want to preserve Rust's safety guarantees.

RefCell can be useful when you need to mutate a value, but you don’t know at compile-time whether you'll need a mutable or immutable reference.
With RefCell, the borrowing rules are enforced dynamically at runtime, giving you more flexibility.

```rs
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);

    // Borrow mutably and change the value inside the RefCell
    *x.borrow_mut() = 10;

    // Borrow immutably and read the value
    println!("The value inside x is: {}", *x.borrow());

    // If you try to borrow mutably while it's already borrowed immutably, it will panic:
    // let mut x_mut = x.borrow_mut(); // This would panic if you already have an immutable borrow.
}
```
