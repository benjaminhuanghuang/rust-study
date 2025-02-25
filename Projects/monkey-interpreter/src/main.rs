use crate::repl::start;
use std::io;

pub mod ast;
pub mod builtins;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod repl;
pub mod token;

fn main() {
  println!("Hello! This is the Monkey programming language!");
  println!("Feel free to type in commands");
  start(io::stdin(), io::stdout());
}
