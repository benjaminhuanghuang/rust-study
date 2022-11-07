use reqwest::{header::HeaderMap, Method};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap};
use url::Url;
use tokio::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiffConfig {
  #[serde(flatten)] //
  pub profiles: HashMap<String, DiffProfile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiffProfile {
  pub req1: RequestProfile,
  pub req2: RequestProfile,
  pub res: ResponseProfile,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestProfile {
  #[serde(with = "http_serde::method", default)]
  pub method: Method,
  pub url: Url,
  #[serde(with = "http_serde::method", default)]
  pub params: Option<serde_json::Value>,
  #[serde(
    skip_serializing_if = "HeaderMap::is_empty",
    with = "http_serde::header_map",
    default
  )]
  pub headers: HeaderMap,
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub body: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseProfile {
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub skip_header: Vec<String>,
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub skip_body: Vec<String>,
}


impl DiffConfig{
  pub async fn load_yaml(path: &str) -> anyhow::Result<Self> {
    let content= fs::read_to_string(path).await?;
    Self::from_yaml(&content)
  }

  pub fn from_yaml(content: &str) -> anyhow::Result<Self> {
    Ok(serde_yaml::from_str(content)?)
  }
}