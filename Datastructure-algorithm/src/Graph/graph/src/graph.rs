use std::collections::HashMap;

#[derive(Debug)]
pub struct NodeNotInGraph;

pub trait Graph {
  fn new() -> Self;
  fn adjacency_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;

  fn add_node(&mut self, node: &str) -> bool {
    match self.adjacency_matrix().get(node) {
      Some(_) => false,
      None => {
        self.adjacency_matrix().insert((*node).to_string(), vec![]);
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
      .and_modify(|e| e.push((to.to_string(), weight)));
  }

  fn neighbors(&mut self, node: &str) -> Result<&Vec<(String, i32)>, NodeNotInGraph> {
    match self.adjacency_matrix().get(node) {
      Some(neighbors) => Ok(neighbors),
      None => Err(NodeNotInGraph),
    }
  }
}
