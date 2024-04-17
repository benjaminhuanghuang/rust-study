mod Backtracking;
mod BacktrackingString;
mod ForLoop;

fn main() {
  let res = ForLoop::solve_n_queens(9);
  println!("{} solutions found", res.len());
  // for sol in res {
  //   for row in sol {
  //     println!("{}", row);
  //   }
  //   println!("-------------");
  // }
}
