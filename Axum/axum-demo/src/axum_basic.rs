use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
  let app = Router::new().route("/hello", get(handler));

  // start server
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8964").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
/*
&'static str means this string is hardcoded into the program and will live for the entire runtime.
No heap allocation (more efficient), Faster execution
*/
async fn handler() -> Html<&'static str> {
  Html("<h1>Hello, Axum!</h1>")
}
