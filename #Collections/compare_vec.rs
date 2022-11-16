#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
  Equal,
  Sublist,
  Superlist,
  Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
  if a == b {
    return Comparison::Equal;
  }
  if a.is_empty() {
    return Comparison::Sublist;
  }
  if b.is_empty() {
    return Comparison::Superlist;
  }
  if a.len() < b.len() {
    //
    if b.windows(a.len()).any(|list| a == list) {
      return Comparison::Sublist;
    }
  }
  if a.len() > b.len() {
    if a.windows(b.len()).any(|list| b == list) {
      return Comparison::Superlist;
    }
  }
  Comparison::Unequal
}
