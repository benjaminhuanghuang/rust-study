use axum::http::HeaderMap;
use axum::response::Html;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct HelloParams {
  name: Option<String>,
}

#[tokio::main]
async fn main() {
  let app = Router::new().route("/header", get(header_extract));

  // start server
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8964").await.unwrap();
  println!("Listening on 127.0.0.1:8964");
  axum::serve(listener, app).await.unwrap();
}

/*
http://localhost:8964/book?id=1&name=Rust

*/
async fn header_extract(headers: HeaderMap) -> Html<String> {
  Html(format!("{headers:#?}"))
}
