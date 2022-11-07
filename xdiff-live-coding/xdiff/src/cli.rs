use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

/// Diff two http requests and compare the difference of the response
#[derive(Parser, Debug)]
#[clap(version, author, about, long_about= None)]
pub(crate) struct Args {
  #[clap(subcommand)]
  pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum Action {
  /// Diff two API responses based on given profile
  Run(RunArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct RunArgs {
  /// Profile name
  #[clap(short, long, value_parser)]
  pub profile: String,

  /// Overrides args. Could be used to override the query, headers and body of the request.
  /// For query params, use `-e key=value`.
  /// For headers, use `-e %key=value`.
  /// For body, use `-e @key=value`.
  #[clap(short, long, value_parser = parse_key_val, number_of_values = 1)]
  pub extra_params: Vec<KeyVal>,

  /// Configuration to use.
  #[clap(short, long, value_parser)]
  pub config: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KeyValType {
  Query,
  Header,
  Body,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyVal {
  key_type: KeyValType,
  key: String,
  value: String,
}

fn parse_key_val(s: &str) -> Result<KeyVal> {
  let mut parts = s.splitn(2, '=');

  let key = parts
    .next()
    .ok_or_else(|| anyhow!("Invalid key value pair"))?
    .trim();
  let value = parts
    .next()
    .ok_or_else(|| anyhow!("Invalid key value pair"))?
    .trim();

  let (key_type, key) = match key.chars().next() {
    Some('%') => (KeyValType::Header, &key[1..]),
    Some('@') => (KeyValType::Body, &key[1..]),
    Some(v) if v.is_ascii_alphabetic() => (KeyValType::Query, key),
    _ => return Err(anyhow!("Invalid key value pair")),
  };

  Ok(KeyVal {
    key_type,
    key: key.to_string(),
    value: value.to_string(),
  })
}