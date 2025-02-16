use clap::Parser;

/// Documents: Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(value_parser)]
  message: Option<String>,

  /// Name of the person to greet
  #[clap(short, long)]
  name: String,

  /// Number of times to greet
  #[clap(short, long, value_parser, default_value_t = 1)]
  count: u8,
}

fn main() {
  let args = Args::parse();

  for _ in 0..args.count {
    println!(
      "{} {}!",
      args.message.as_deref().unwrap_or("hello"),
      args.name
    )
  }
}

/*

 Usage:

 cargo run -- hello -n Ben -c 3

 cargo run -- "Good evening"  -n Ben -c 2
*/
