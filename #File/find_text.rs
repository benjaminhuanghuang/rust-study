use glob::glob;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  for item in glob("**/*.txt")? {
    println!("{}", item?.display());
  }

  Ok(())
}
