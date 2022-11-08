use reqwest::{header::{self, HeaderMap, HeaderName, HeaderValue}, Client, Method, Response,};
use serde::{Deserialize, Serialize};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::fmt::Write;
use std::str::FromStr;
use tokio::fs;
use url::Url;
use anyhow::Result;

use crate::ExtraArgs;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestProfile {
  #[serde(with = "http_serde::method", default)]
  pub method: Method,
  pub url: Url,
  #[serde(skip_serializing_if = "Option::is_none", default)]
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

#[derive(Debug)]
pub struct ResponseExt(Response);

impl RequestProfile {
  pub async fn send(&self, args: &ExtraArgs) -> Result<ResponseExt> {
    let (headers, query, body) = self.generate(args)?;
    let client = Client::new();
    let req = client
        .request(self.method.clone(), self.url.clone())
        .query(&query)
        .headers(headers)
        .body(body)
        .build()?;

    let res = client.execute(req).await?;
    Ok(ResponseExt(res))
}

fn generate(&self, args: &ExtraArgs) -> Result<(HeaderMap, serde_json::Value, String)> {
  let mut headers = self.headers.clone();
  //?
  let mut query = self.params.clone().unwrap_or_else(|| json!({}));
  let mut body = self.body.clone().unwrap_or_else(|| json!({}));

  for (k, v) in &args.headers {
      headers.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
  }

  if !headers.contains_key(header::CONTENT_TYPE) {
      headers.insert(
          header::CONTENT_TYPE,
          HeaderValue::from_static("application/json"),
      );
  }

  for (k, v) in &args.query {
      query[k] = v.parse()?;
  }

  for (k, v) in &args.body {
      body[k] = v.parse()?;
  }

  let content_type = get_content_type(&headers);
  match content_type.as_deref() {
      Some("application/json") => {
          let body = serde_json::to_string(&body)?;
          Ok((headers, query, body))
      }
      Some("application/x-www-form-urlencoded" | "multipart/form-data") => {
          let body = serde_urlencoded::to_string(&body)?;
          Ok((headers, query, body))
      }
      _ => Err(anyhow::anyhow!("unsupported content-type")),
  }
}
}

fn get_content_type(headers: &HeaderMap) -> Option<String> {
  headers
      .get(header::CONTENT_TYPE)
      .and_then(|v| v.to_str().unwrap().split(';').next().map(|v| v.to_string()))
}
