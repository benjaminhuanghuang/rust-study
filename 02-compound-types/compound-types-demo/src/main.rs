struct Person {
  name: String,
  age: u8,
}

// tuple type struct
struct Point2D(u32, u32);

// Enum
// Define type for each variant
enum Direction {
  North(String),
  South(String),
  East(String),
  West(String),
}

// each variant has different type
enum WebEvent {
  PageLoad,
  KeyPress(char),
  Paste(String),
  Click { x: i64, y: i64 },
}

// Use <T>
enum Option<T> {
  Some(T),
  None,
}

enum Species {
  Crab,
  Octopus,
  Fish,
  Clam,
}

enum PoisonType {
  Acidic,
  Painful,
  Lethal,
}

enum Size {
  Big,
  Small,
}

enum Weapon {
  Claw(i32, Size),
  Poison(PoisonType),
  None,
}

struct SeaCreature {
  species: Species,
  name: String,
  arms: i32,
  legs: i32,
  weapon: Weapon,
}

fn main() {
  let arr: [u32; 3] = [1, 2, 3];

  // Tuple
  let tuple: (bool, u16, u8) = (true, 2, 3);
  let first = tuple.0;

  //
  let p = Person {
    name: String::from("Abe"),
    age: 100,
  };

  println!("Person name is :{}", p.name);

  // tuple type struct
  let point = Point2D(1, 2);
  println!("point contains {:?}, {:?}", point.0, point.1);

  // destruct the tuple struct
  let Point2D(x, y) = point;
  println!("point contains {:?}, {:?}", x, y);

  // Use enum
  let north = Direction::North(String::from("North"));

  let quit = WebEvent::KeyPress('q');

  let result = Some(1);
}
