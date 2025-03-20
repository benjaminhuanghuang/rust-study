use std::collections::HashMap;

use axum::extract::Query;
use axum::response::Html;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct HelloParams {
  name: Option<String>,
}

#[tokio::main]
async fn main() {
  let app = Router::new().route("/book", get(query_extract));

  // start server
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8964").await.unwrap();
  println!("Listening on 127.0.0.1:8964");
  axum::serve(listener, app).await.unwrap();
}

/*
http://localhost:8964/book?id=1&name=Rust
Query<HashMap<String, String>> is an Axum extractor that automatically parses query parameters into a HashMap<String, String>.
params holds all query parameters as key-value pairs (both keys and values are String).
The Query(params) syntax destructures the extracted query parameters into the params variable.
*/
async fn query_extract(Query(params): Query<HashMap<String, String>>) -> Html<String> {
  Html(format!("{params:#?}"))
}
