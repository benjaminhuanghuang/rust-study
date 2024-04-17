mod Backtracking;
mod solution1;

fn main() {
  let res = solution1::solve_n_queens(9);
  println!("{} solutions found", res.len());
  // for sol in res {
  //   for row in sol {
  //     println!("{}", row);
  //   }
  //   println!("-------------");
  // }
}
