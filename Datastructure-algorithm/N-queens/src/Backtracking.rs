/*
  51. N-Queens
  placing n queens on an n x n chessboard
  https://leetcode.com/problems/n-queens/description/

  Backtracking
  Time complexity: O(N!)

*/
pub fn solve_n_queens(n: i32) -> Vec<Vec<i32>> {
  let mut solutions: Vec<Vec<i32>> = Vec::new();
  backtrack(n, 0, &mut Vec::<i32>::new(), &mut solutions);
  solutions
}

fn backtrack(n: i32, row: i32, colPlacements: &mut Vec<i32>, solutions: &mut Vec<Vec<i32>>) {
  if row == n {
    // finish, add colPlacements to solution
    solutions.push(colPlacements);
    return;
  }

  for col in 0..n {
    colPlacements.push(col); // place the queen
    if isValid(colPlacements) {
      backtrack(n, row + 1, colPlacements, solutions);
    }
    colPlacements.pop(); // remove the queen
  }
}

fn isValid(colPlacements: &Vec<i32>) -> bool {
  let lastRow = colPlacements.len() - 1;
  let lastCol = colPlacements[lastRow];

  for row in 0..lastRow {
    let col = colPlacements[row];
    if col == lastCol || (col - lastCol).abs() == lastRow - row {
      return false;
    }
  }
  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_assert() {
    let res = solve_n_queens(1);
    assert_eq!(1, res.len());

    let res = solve_n_queens(3);
    assert_eq!(0, res.len());

    let res = solve_n_queens(9);
    assert_eq!(352, res.len());
  }
}
