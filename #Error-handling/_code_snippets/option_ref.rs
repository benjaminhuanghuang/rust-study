struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let y: Option<Point> = Some(Point { x: 100, y: 200 });

  match y {
    //The compiler says a partial move happened in the `match` statement.
    Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
    _ => println!("no match"),
  }
  y; // Fix without deleting this line.
}
