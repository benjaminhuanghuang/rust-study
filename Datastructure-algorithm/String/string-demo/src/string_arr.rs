fn greet() {
  let regions = ["a", "b"];

  for region in regions.iter() {
    println!("{}", &region); // read-only borrow
  }
}
