use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
  let routes_hello = Router::new().route(
    "/hello",
    get(|| async { Html("<h1>Hello, Axum!</h1>".to_string()) }),
  );

  // start server
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8964").await.unwrap();
  axum::serve(listener, routes_hello).await.unwrap();
}
