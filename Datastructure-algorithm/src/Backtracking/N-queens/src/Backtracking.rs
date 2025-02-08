/*
  51. N-Queens
  placing n queens on an n x n chessboard
  https://leetcode.com/problems/n-queens/description/

  Backtracking
  Time complexity: O(N!)

*/
pub struct Solution;

impl Solution {
  pub fn solve_n_queens(n: i32) -> Vec<Vec<i32>> {
    let mut solutions: Vec<Vec<i32>> = Vec::new();
    let mut col_placements = Vec::<i32>::new();
    Self::backtrack(n, 0, &mut col_placements, &mut solutions);
    solutions
  }

  fn backtrack(n: i32, row: i32, col_placements: &mut Vec<i32>, solutions: &mut Vec<Vec<i32>>) {
    if row == n {
      // Finish, add colPlacements to solution
      solutions.push(col_placements.clone());
      return;
    }

    for col in 0..n {
      col_placements.push(col); // Place the queen
      if Self::is_valid(col_placements) {
        Self::backtrack(n, row + 1, col_placements, solutions);
      }
      col_placements.pop(); // Remove the queen
    }
  }

  fn is_valid(col_placements: &Vec<i32>) -> bool {
    let last_row = col_placements.len() - 1;
    let last_col = col_placements[last_row];

    for row in 0..last_row {
      let col = col_placements[row];
      if col == last_col || (col - last_col).abs() == last_row as i32 - row as i32 {
        return false;
      }
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_assert() {
    let res = Solution::solve_n_queens(1);
    assert_eq!(1, res.len());

    let res = Solution::solve_n_queens(3);
    assert_eq!(0, res.len());

    let res = Solution::solve_n_queens(9);
    assert_eq!(352, res.len());
  }
}
