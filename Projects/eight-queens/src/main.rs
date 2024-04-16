/*
  51. N-Queens
  placing n queens on an n x n chessboard
  https://leetcode.com/problems/n-queens/description/
*/
fn sq_to_idx(col: i32, row: i32, n: i32) -> i32 {
  n * row + col
}
fn idx_to_sq(idx: i32, n: i32) -> (i32, i32) {
  (idx / n, idx % n) // row, col or x, y
}

/*
  items in the solution vector are the col index of the queens
*/
fn check(sol: &Vec<i32>, n: i32) -> bool {
  for i in 0..sol.len() {
    let (x1, y1) = idx_to_sq(sol[i], n);
    for j in i + 1..sol.len() {
      let (x2, y2) = idx_to_sq(sol[j], n);
      if x1 == x2 || y1 == y2 || (x1 - x2).abs() == (y1 - y2).abs() {
        return false;
      }
    }
  }
  true
}

fn generate(curr: &Vec<Vec<i32>>, n: i32) -> Vec<Vec<i32>> {
  let mut ret: Vec<Vec<i32>> = Vec::new();

  for sol in curr {
    let rank = sol.len() as i32;
    for file in 0..n {
      let mut new_sol = sol.clone();
      new_sol.push(sq_to_idx(rank, file, n));
      if check(&new_sol, n) {
        ret.push(new_sol);
      }
    }
  }
  ret
}

fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
  let mut solutions: Vec<Vec<i32>> = Vec::new();
  for file in 0..n {
    solutions.push(vec![sq_to_idx(0, file, n)]);
  }
  for _ in 1..n {
    solutions = generate(&solutions, n);
  }

  let mut ret: Vec<Vec<String>> = Vec::new();
  for sol in &solutions {
    let mut sol_str: Vec<String> = Vec::new();
    for idx in sol {
      let (_, file) = idx_to_sq(*idx, n);
      let st = (0..n).map(|x| if x == file { 'Q' } else { '.' }).collect();
      sol_str.push(st);
    }
    ret.push(sol_str);
  }

  ret
}

fn main() {
  let res = solve_n_queens(9);
  println!("{} solutions found", res.len());
  // for sol in res {
  //   for row in sol {
  //     println!("{}", row);
  //   }
  //   println!("-------------");
  // }
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
