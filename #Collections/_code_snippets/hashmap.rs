use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Fruit {
  Apple,
  Banana,
  Mango,
  Lychee,
  Pineapple,
}

//------------------------------------------------
// Create
//------------------------------------------------
let mut basket = HashMap::<Fruit, u32>::new();
let mut basket = HashMap::new();
let mut scores: HashMap<String, u32> = HashMap::new();


//------------------------------------------------
// Insert
//------------------------------------------------
basket.insert(Fruit::Apple, 4);
basket.insert(Fruit::Mango, 2);
basket.insert(Fruit::Lychee, 5);


//------------------------------------------------
// Get entry, Insert / Modify
//------------------------------------------------
let r = "England,France,4,2";
let v: Vec<&str> = r.split(',').collect();
let team_1_name = v[0].to_string();
let team_1_score: u8 = v[2].parse().unwrap();
let team_2_name = v[1].to_string();
let team_2_score: u8 = v[3].parse().unwrap();

let t1 = scores.entry(team_1_name.clone()).or_insert(Team {
  name: team_1_name,
  goals_scored: 0,
  goals_conceded: 0,
});
(*t1).goals_scored += team_1_score;
(*t1).goals_conceded += team_2_score;

let t2 = scores.entry(team_2_name.clone()).or_insert(Team {
  name: team_2_name,
  goals_scored: 0,
  goals_conceded: 0,
});
(*t2).goals_conceded += team_1_score;
(*t2).goals_scored += team_2_score;

//------------------------------------------------
// Count
//------------------------------------------------
let count_fruit_kinds = basket.len();

//------------------------------------------------
// Check
//------------------------------------------------
let fruit_kinds = vec![Fruit::Lychee,Fruit::Mango]
for fruit in fruit_kinds {
  if !basket.contains_key(&fruit) {
      basket.insert(fruit, 10);
  }
}
//------------------------------------------------
// Values
//------------------------------------------------


//------------------------------------------------
// Key-value
//------------------------------------------------
assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
