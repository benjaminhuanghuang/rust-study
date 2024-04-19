use crate::graph::Graph;
use std::collections::HashMap;

pub struct UndirectedGraph {
  // key: node, value: list of (node, weight)
  adjacency_matrix: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
  fn new() -> Self {
    UndirectedGraph {
      adjacency_matrix: HashMap::new(),
    }
  }

  fn adjacency_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
    &mut self.adjacency_matrix
  }

  fn add_edge(&mut self, edge: (&str, &str, i32)) {
    let (from, to, weight) = edge;
    self.add_node(from);
    self.add_node(to);
    self
      .adjacency_matrix()
      .entry(from.to_string())
      .and_modify(|e| e.push((to.to_string(), weight)));
    self
      .adjacency_matrix()
      .entry(to.to_string())
      .and_modify(|e| e.push((from.to_string(), weight)));
  }
}
#[cfg(test)]
mod test_undirected_graph {
  use super::*;

  #[test]
  fn test_neighbors() {
    let mut graph = UndirectedGraph::new();
    graph.add_edge(("A", "B", 1));
    graph.add_edge(("B", "C", 2));
    graph.add_edge(("C", "A", 3));

    assert_eq!(
      graph.neighbors("A").unwrap(),
      &vec![(String::from("B"), 1), (String::from("C"), 3)]
    );
  }
}
