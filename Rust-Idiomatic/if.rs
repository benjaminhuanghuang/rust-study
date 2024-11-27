fn animal_habitat(animal: &str) -> &str {
  let identifier = if animal == "crab" {
    1
  } else if animal == "gopher" {
    2
  } else if animal == "snake" {
    3
  } else {
    10
  };

  // Don't change the expression below!
  if identifier == 1 {
    "Beach"
  } else if identifier == 2 {
    "Burrow"
  } else if identifier == 3 {
    "Desert"
  } else {
    "Unknown"
  }
}

fn picky_eater(food: &str) -> &str {
  if food == "strawberry" {
    "Yummy!"
  } else {
    "1"
  }
}
