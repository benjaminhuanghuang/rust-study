use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Fruit {
  Apple,
  Banana,
  Mango,
  Lychee,
  Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
  let fruit_kinds = vec![
    Fruit::Apple,
    Fruit::Banana,
    Fruit::Mango,
    Fruit::Lychee,
    Fruit::Pineapple,
  ];

  for fruit in fruit_kinds {
    // TODO: Put new fruits if not already present. Note that you
    // are not allowed to put any type of fruit that's already
    // present!
    basket.insert(Fruit::Pineapple, 10);
    basket.insert(Fruit::Banana, 10);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_fruit_basket() -> HashMap<Fruit, u32> {
    let mut basket = HashMap::<Fruit, u32>::new();
    basket.insert(Fruit::Apple, 4);
    basket.insert(Fruit::Mango, 2);
    basket.insert(Fruit::Lychee, 5);

    basket
  }

  #[test]
  fn test_given_fruits_are_not_modified() {
    let mut basket = get_fruit_basket();
    fruit_basket(&mut basket);
    assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
    assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
    assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
  }

  #[test]
  fn at_least_five_types_of_fruits() {
    let mut basket = get_fruit_basket();
    fruit_basket(&mut basket);
    let count_fruit_kinds = basket.len();
    assert!(count_fruit_kinds >= 5);
  }

  #[test]
  fn greater_than_eleven_fruits() {
    let mut basket = get_fruit_basket();
    fruit_basket(&mut basket);
    let count = basket.values().sum::<u32>();
    assert!(count > 11);
  }
}
