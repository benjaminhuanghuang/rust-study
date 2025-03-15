/*
Usage:
cargo run -- --help

cargo run -- -f John -l Doe

# use the sub command
cargo run -- register-pet --pet-name Cheye

positional argument: does not have a name
  .arg(Arg::new("firstname"))


Options:


*/
use clap::{command, Arg, ArgGroup, Command};

fn main() {
  let match_result = command!()
    .about("This app is for demonstration purposes")
    .subcommand(
      Command::new("register-pet")
        .arg(Arg::new("petname").long("pet-name").short('n'))
        .arg(Arg::new("fluffy").long("fluffy")),
    )
    .group(
      ArgGroup::new("person-register")
        .arg("firstname")
        .arg("lastname"),
    )
    .arg(
      Arg::new("firstname")
        .short('f')
        .long("first-name")
        .aliases(["fname", "firstname"])
        .required(true)
        .help("The first name of the user"), //.conflicts_with("lastname"), // conflict with lastname
    )
    .arg(
      Arg::new("lastname")
        .required(true)
        .help("The last name of the user"),
    )
    .get_matches();

  println!(
    "match_result: {:?}",
    match_result.get_one::<String>("firstname").unwrap()
  );

  println!(
    "match_result: {:?}",
    match_result
      .get_one::<String>("firstname")
      .unwrap_or(&"default_value".to_string())
  );
}
