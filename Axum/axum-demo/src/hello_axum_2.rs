use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
  let routes_hello = Router::new().route("/hello", get(handler_hello));

  // start server
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8964").await.unwrap();
  axum::serve(listener, routes_hello).await.unwrap();
}

async fn handler_hello() -> Html<String> {
  Html("<h1>Hello, Axum 2!</h1>".to_string())
}
