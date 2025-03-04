/*
Rust has three struct types: a classic C struct, a tuple struct, and a unit struct.
*/
struct ColorRegularStruct {
  pub green: i32,
  pub red: i32,
  pub blue: i32,
}


let green = ColorRegularStruct {
  green: 255,
  red: 0,
  blue: 0,
};

struct ColorTupleStruct(i32, i32, i32);
let green = ColorTupleStruct(0, 255, 0);


#[derive(Debug)]
struct UnitStruct;
let unit_struct = UnitStruct;


