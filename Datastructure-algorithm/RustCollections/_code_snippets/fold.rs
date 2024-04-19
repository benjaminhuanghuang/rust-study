/*
Sum

fold 第一个是初始值，第二个是一个闭包，闭包第一个参数是一个累计值，第二个参数是本次迭代元素的引用，返回值作为下一次迭代的累计值。
其中acc在第一次迭代的时候就是初始值0，也就是fold函数第一个参数，每次迭代都会返回acc+x作为下一次acc的值，
也就是每次迭代都会加上这次迭代的结果

*/

 let vec = vec![1, 2, 3, 4, 5];
 let res = vec.iter().fold(0, |acc, x| acc + x);
 eprint!("{}", res);



pub fn factorial(num: u64) -> u64 {
  (1..=num).fold(1, |acc, elem| acc * elem)
}
