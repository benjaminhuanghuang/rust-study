use anyhow::{Context, Result};
use reqwest::{header::HeaderMap, Method};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;
use url::Url;
use crate::req::RequestProfile;

use crate::ExtraArgs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiffConfig {
  #[serde(flatten)] //
  pub profiles: HashMap<String, DiffProfile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    #[serde(skip_serializing_if = "is_default", default)]
    pub res: ResponseProfile,
}

pub fn is_default<T: Default + PartialEq>(v: &T) -> bool {
  v == &T::default()
}


#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct ResponseProfile {
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub skip_header: Vec<String>,
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub skip_body: Vec<String>,
}

impl DiffConfig {
  pub fn new(profiles: HashMap<String, DiffProfile>) -> Self {
    Self { profiles }
}

  pub async fn load_yaml(path: &str) -> anyhow::Result<Self> {
    let content = fs::read_to_string(path).await?;
    Self::from_yaml(&content)
  }

  pub fn from_yaml(content: &str) -> anyhow::Result<Self> {
    Ok(serde_yaml::from_str(content)?)
  }

  pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
    self.profiles.get(name)
  }
}

impl DiffProfile {
  pub fn new(req1: RequestProfile, req2: RequestProfile, res: ResponseProfile) -> Self {
    Self { req1, req2, res }
  }

  pub async fn diff(&self, args: ExtraArgs) -> Result<String> {
    // println!("profile: {:?}", self);
    // println!("args: {:?}", args);
    // Ok("".to_string())
    let res1 = self.req1.send(&args).await?;
    let res2 = self.req2.send(&args).await?;

    let text1 = res1.get_text(&self.res).await?;
    let text2 = res2.get_text(&self.res).await?;

    diff_text(&text1, &text2)

  }
}
