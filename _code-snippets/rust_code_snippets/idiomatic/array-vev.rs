fn array_and_vec() -> ([i32; 4], Vec<i32>) {
  let a = [10, 20, 30, 40]; // Array

  // Use the vector macro.
  // let v = ???;
  let v = vec![a];
  let v = Vec::from(a);
  let v = a.to_vec();

  (a, v)
}
