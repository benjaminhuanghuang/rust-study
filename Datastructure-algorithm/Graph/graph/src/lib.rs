mod directed_graph;
mod graph;
mod undirected_graph;
use std::collections::{HashSet, LinkedList, VecDeque};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(u8);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u8, u8);

#[derive(Clone)]
pub struct Graph {
  pub vertices: Vec<Vertex>,
  pub edges: Vec<Edge>,
}

impl Graph {
  pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
    Graph { vertices, edges }
  }
}

impl From<u8> for Vertex {
  fn from(item: u8) -> Self {
    Vertex(item)
  }
}

impl Vertex {
  pub fn value(&self) -> u8 {
    self.0
  }

  pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
    graph
      .edges
      .iter()
      .filter(|e| e.0 == self.0)
      .map(|e| e.1.into())
      .collect()
  }
}

impl From<(u8, u8)> for Edge {
  fn from(item: (u8, u8)) -> Self {
    Edge(item.0, item.1)
  }
}

// Use a stack
pub fn depth_first_search_history(
  graph: &Graph,
  start: Vertex,
  objective: Vertex,
) -> Option<Vec<u8>> {
  let mut visited: HashSet<Vertex> = HashSet::new();
  let mut history: Vec<u8> = Vec::new();
  let mut queue: VecDeque<Vertex> = VecDeque::new(); // Use a stack

  queue.push_back(start);
  while let Some(curr_vertex) = queue.pop_front() {
    history.push(curr_vertex.value());
    if curr_vertex == objective {
      return Some(history);
    }
    // Go through all the neighbors of the current vertex
    for neighbor in curr_vertex.neighbors(graph).into_iter().rev() {
      if !visited.contains(&neighbor) {
        queue.push_front(neighbor);
        visited.insert(neighbor);
      }
    }
  }
  None
}

// Use a stack
pub fn depth_first_search(graph: &Graph, start: Vertex, objective: Vertex) -> Option<Vec<u8>> {
  let mut visited: HashSet<Vertex> = HashSet::new();
  let mut stack: LinkedList<Vertex> = LinkedList::new();
  let mut result: Vec<u8> = Vec::new();

  visited.insert(start);
  stack.push_back(start);

  while let Some(curr_vertex) = stack.pop_back() {
    result.push(curr_vertex.value());
    if curr_vertex == objective {
      return Some(result);
    }
    // Go through all the neighbors of the current vertex
    for neighbor in curr_vertex.neighbors(graph).into_iter() {
      if !visited.contains(&neighbor) {
        stack.push_back(neighbor);
        visited.insert(neighbor);
      }
    }
  }
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_dfs_history() {
    let vertices = vec![1, 2, 3, 4, 5, 6, 7];
    let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];
    let root = 1;
    let objective = 7;
    let expect = vec![1, 2, 4, 5, 3, 6, 7];

    let graph = Graph::new(
      vertices.into_iter().map(|v| v.into()).collect(),
      edges.into_iter().map(|e| e.into()).collect(),
    );
    let result = depth_first_search_history(&graph, root.into(), objective.into());
    assert_eq!(result, Some(expect));
  }

  #[test]
  fn test_dfs() {
    let vertices = vec![1, 2, 3, 4, 5, 6, 7];
    let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];
    let root = 1;
    let objective = 7;
    let expect = vec![1, 3, 7];

    let graph = Graph::new(
      vertices.into_iter().map(|v| v.into()).collect(),
      edges.into_iter().map(|e| e.into()).collect(),
    );
    let result = depth_first_search(&graph, root.into(), objective.into());
    assert_eq!(result, Some(expect));
  }
}
