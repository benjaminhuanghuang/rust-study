fn main() {
  let x = 42;

  match x {
      0 => {
          println!("found zero");
      }
      // 我们可以匹配多个值
      1 | 2 => {
          println!("found 1 or 2!");
      }
      // 我们可以匹配迭代器
      3..=9 => {
          println!("found a number 3 to 9 inclusively");
      }
      // 我们可以将匹配数值绑定到变量
      matched_num @ 10..=100 => {
          println!("found {} number between 10 to 100!", matched_num);
      }
      // 这是默认匹配，如果没有处理所有情况，则必须存在该匹配
      _ => {
          println!("found something else!");
      }
  }
}
