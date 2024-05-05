# Generics in Rust

- Generic structs
  
```rust
struct Point<T> {
    x: T,
    y: T,
}

let c1 = Point::<i32> { x: 10, y: 20 };

// compiler can infer the type
let c2 = Point{ x: 10, y: 20 };
```

- Generic enums
- Generic traits
- Generic functions
- 
```rust 
fn process array<T>(arr: & [T]) {
  printin! ("() elements, () bytes each",
        arr.len (),
        std: :mem: :size_of:: <T>());
}


let a1 = [1, 2, 3, 4, 5];
process_array::<i32>(&a1);

let a2 = [1.0, 2.0, 3.0, 4.0, 5.0];
process_array(&a2);        // compiler can infer the type implicitly
```

## Type Constraints

```ts
fn display<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item);   // T must implement Debug trait
}
```

## Eq

Symmetrical, transitive, and reflexive equality

Eq inherits basic equality capabilities from PartialEq
  • Inherits eq () for == tests
  • Inherits ne () for ! = tests

Eq promises equivalence relations:
• Reflexive: x==x
• Symmetric: x==y implies y==x
• Transitive: x==y and y==z implies x==z

## PartialOrdc

```rust
struct Angle {
  degrees: 132
}

impl PartialOrd for Angle {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
  let dl = self.degrees & 360;
  let d2 = other.degrees % 360;
  Some (dl. cmp (&d2) )
  }
}

impl PartialEq for Angle {
  fn eq(&self, other: &Self) -> bool i{
    self.degrees % 360 == other.degrees % 360
  }
}
```
