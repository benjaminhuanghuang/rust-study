use crate::graph::Graph;
use std::collections::HashMap;

pub struct DirectedGraph {
  // key: node, value: list of (node, weight)
  adjacency_matrix: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for DirectedGraph {
  fn new() -> Self {
    DirectedGraph {
      adjacency_matrix: HashMap::new(),
    }
  }

  fn adjacency_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
    &mut self.adjacency_matrix
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_directed() {
    let mut graph = DirectedGraph::new();
    graph.add_edge(("A", "B", 1));
    graph.add_edge(("B", "C", 2));
    graph.add_edge(("C", "A", 3));
    graph.add_edge(("B", "A", 4)); // directed edge

    assert_eq!(
      graph.adjacency_matrix().get("A").unwrap(),
      &vec![(String::from("B"), 1)]
    );
  }
}
