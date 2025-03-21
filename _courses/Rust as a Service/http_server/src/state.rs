use std::sync::Arc;

use axum::extract::State;
use axum::response::Html;
use axum::{routing::get, Router};

struct MyConfig {
  config_string: String,
}

#[tokio::main]
async fn main() {
  let shared_state = Arc::new(MyConfig {
    config_string: "Hello, World!".to_string(),
  });

  let app = Router::new()
    .route("/", get(handler))
    .with_state(shared_state);

  // start server
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8964").await.unwrap();
  println!("Listening on 127.0.0.1:8964");
  axum::serve(listener, app).await.unwrap();
}

/*
http://localhost:8964/book?id=1&name=Rust

*/
async fn handler(State(config): State<Arc<MyConfig>>) -> Html<String> {
  Html(format!("<><h1>{}</h1>", config.config_string))
}
