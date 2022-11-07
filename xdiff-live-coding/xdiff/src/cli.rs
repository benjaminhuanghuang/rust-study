use clap::{Parser, Subcommand};
use anyhow::Result;


/// Diff two http requests and compare the difference of the response
#[derive(Parse, Debug)]
#[clap(version, author, about, long_about= None)]
pub(crate) struct Args {
  #[clap(subcommand)]
  pub action: Action;
}

#[derive(subcommand, Debug, Clone)]
pub(crate) enum Action {
  /// Diff two API responses based on given profile
  Run(DiffArgs)
}



#[derive(Parser, Debug, Clone)]
pub(crate) struct RunArgs {
  #[clap(short, long, value_parser)]
  pub profile: String,

  /// Overrides args
  #[clap(short, long, value_parser= parse_key_value, number_of_values=1)] 
  extra_params: Vec<KeyVal>,
}


#[derive(Debug, Clone)]
pub(crate) struct KeyVal {
  key: String,
 value: Vec<KeyVal>,
}


fn parse_key_value(s:&str) -> Result<keyVal> {
  todo!()
}


