use anyhow::Result;
use xdiff::{LoadConfig, RequestConfig};

fn main() -> Result<()> {
  let content: &str = include_str!("../fixtures/test.yml");
  let config = RequestConfig::from_yaml(content)?;
  println!("{:#?}", config);

  Ok(())
}
