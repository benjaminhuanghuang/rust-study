use std::collections::HashMap;

pub struct UndirectedGraph {
  // key: node, value: list of (node, weight)
  adjacency_matrix: HashMap<String, Vec<(String, i32)>>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
