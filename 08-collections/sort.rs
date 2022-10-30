fn main() {
  let mut vec = vec![1, 23, 42, 23, 45, 223, 211, 122, 233, 799, 123];

  vec.sort();

  println!("Sorted: {:?}", vec);

  // sort function
  let mut vec = vec![23.12, 3.44, 5.55, 34.90, 2.0];

  vec.sort_by(|x, y| x.partial_cmp(y).unwrap());

  println!("Sorted: {:?}", vec);
}
