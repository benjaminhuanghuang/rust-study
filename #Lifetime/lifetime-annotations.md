
```
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
  *i + *j
}
```

<'a, 'b> declares two lifetime variables, 'a and 'b, within the scope of add_with_lifetimes(). These are normally spoken as lifetime a and lifetime b.

i: &'a i32 binds lifetime variable 'a to the lifetime of i. The syntax reads as “parameter i is a reference to an i32 with lifetime a.”

j: &'b i32 binds the lifetime variable 'b to the lifetime of j. The syntax reads as “parameter j is a reference to an i32 with lifetime b.”