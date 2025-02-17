use std::io::{Stdin, Stdout, Write};

use crate::evaluator::Evaluator;
use crate::lexer;
use crate::parser::Parser;

pub fn start(stdin: Stdin, mut stdout: Stdout) {
  let evaluator = Evaluator::new();
  loop {
    write!(stdout, ">> ").expect("should have written prompt string >>");
    stdout.flush().expect("should have flushed stdout");

    let mut input = String::new();
    if let Err(e) = stdin.read_line(&mut input) {
      writeln!(stdout, "Error: {e}").expect("should have written error message");

      return;
    }
    let lexer = lexer::Lexer::new(input.as_str());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().expect("error parsing program");
    if parser.errors().len() != 0 {
      print_parse_errors(&stdout, parser.errors());
      continue;
    }

    // let parsed_program_string = program.print_string();
    // writeln!(stdout, "{parsed_program_string}")
    //   .expect("parsed program should have been written to stdout");

    let evaluated = evaluator.eval_program(program);
    writeln!(stdout, "{evaluated}").expect("evaluated program should have been written to stdout");
  }
}

fn print_parse_errors(mut stdout: &Stdout, errors: &Vec<String>) {
  writeln!(stdout, "Woops! We ran into some monkey business here!").unwrap();

  for error in errors {
    writeln!(stdout, "\t-> {error}").expect("Error should have been written to stdout");
  }
}
