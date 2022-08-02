fn main() {
  let args = std::env::args().collect::<Vec<String>>();

  if args.len() != 2 {
    eprintln!("stat: missing operand");
    eprintln!("Try 'stat --help' for more information");
    return;
  }
}
