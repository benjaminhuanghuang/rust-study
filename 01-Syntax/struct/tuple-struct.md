##
Store a collection of mixed data without named fields

Distinguishable as unique data type

```
  struct Color(u8,u8, u8);

  let red = Color(255, 0, 0);

  println!("{}", red.0)

```

One common of a tuple struct is called the newtype patter.
Add meaning to a existing type by wrapped it in a tuple struct
```
struct Meters(u8);

let d1 = Meters(3);
let d1 = 7; // should be Meters(3);

fn add_distances(d1: Meters, d2: Meters){

}
```

