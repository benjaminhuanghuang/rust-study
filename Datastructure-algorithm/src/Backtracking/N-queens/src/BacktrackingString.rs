/*
  51. N-Queens
  placing n queens on an n x n chessboard
  https://leetcode.com/problems/n-queens/description/

  Backtracking
  Time complexity: O(N!)

*/
pub struct Solution;

impl Solution {
  /*
    expected result: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
  */
  pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut solutions: Vec<Vec<String>> = Vec::new();
    let mut solution = vec![".".repeat(n as usize); n as usize]; // ["....","....","....","...."]
    Self::backtrack(n, 0, &mut solution, &mut solutions);
    solutions
  }

  fn backtrack(n: i32, row: i32, solution: &mut Vec<String>, solutions: &mut Vec<Vec<String>>) {
    if row == n {
      // Finish, add colPlacements to solution
      solutions.push(solution.clone());
      return;
    }

    for col in 0..n {
      if Self::is_valid(n, solution, row, col) {
        solution[row as usize] =
          Self::set_board(solution[row as usize].as_str(), col as usize, 'Q');
        Self::backtrack(n, row + 1, solution, solutions);
        solution[row as usize] =
          Self::set_board(solution[row as usize].as_str(), col as usize, '.');
      }
    }
  }

  // Check if the Q can be placed at (row, col)
  fn is_valid(n: i32, solution: &[String], queen_row: i32, queen_col: i32) -> bool {
    for row in 0..queen_row {
      for col in 0..n {
        if solution[row as usize].chars().nth(col as usize).unwrap() == 'Q'
          && (col == queen_col || (col - queen_col).abs() == queen_row as i32 - row as i32)
        {
          return false;
        }
      }
    }
    true
  }

  fn set_board(row: &str, col: usize, c: char) -> String {
    let mut chars: Vec<char> = row.chars().collect();
    chars[col] = c;
    chars.into_iter().collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_assert() {
    let res = Solution::solve_n_queens(1);
    assert_eq!(1, res.len());
  }
}
