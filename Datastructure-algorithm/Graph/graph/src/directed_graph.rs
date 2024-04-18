use std::collections::HashMap;

#[derive(Debug)]
pub struct NodeNotInGraph;

pub struct DirectedGraph {
  // key: node, value: list of (node, weight)
  adjacency_matrix: HashMap<String, Vec<(String, i32)>>,
}

pub trait Graph {
  fn new() -> Self;
  fn adjacency_matrix(&self) -> &HashMap<String, Vec<(String, i32)>>;

  fn add_node(&mut self, node: &str) -> bool {
    match self.adjacency_matrix().get(node) {
      Some(_) => false,
      None => {
        self.adjacency_matrix().insert(*node, vec![]);
        true
      }
    }
  }

  fn add_edge(&mut self, edge: (&str, &str, i32)) {
    let (from, to, weight) = edge;
    self.add_node(from);
    self.add_node(to);
    self
      .adjacency_matrix()
      .entry(from.to_string())
      .add_modify(|e| e.push((to.to_string(), weight)));
  }

  fn neighbors(&self, node: &str) -> Result<&Vec<(String, i32)>, NodeNotInGraph> {
    match self.adjacency_matrix().get(node) {
      Some(neighbors) => Ok(neighbors),
      None => Err(NodeNotInGraph),
    }
  }
}

impl Graph for DirectedGraph {
  fn new() -> Self {
    DirectedGraph {
      adjacency_matrix: HashMap::new(),
    }
  }

  fn adjacency_matrix(&self) -> &HashMap<String, Vec<(String, i32)>> {
    &self.adjacency_matrix
  }
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
