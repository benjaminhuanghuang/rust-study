# Create struct

To create a struct instance:
• Use the struct name, followed by {}
• Inside the {}, initialize all fields (in any order)

```rust
let el1 = Employee {
  name: "John".to_string(),
  age: 30,
  salary: 1000.0,
  is_active: true,
};

// Mutable struct instance
let mut el2 = Employee {
  name: "John".to_string(),
  age: 30,
  salary: 1000.0,
  is_active: true,
};
```

NOTE: struct instances are stack-allocated.
