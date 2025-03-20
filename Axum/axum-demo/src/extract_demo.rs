use axum::extract::Path;
use axum::response::Html;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct HelloParams {
  name: Option<String>,
}

#[tokio::main]
async fn main() {
  let app = Router::new().route("/book/{id}", get(path_extract));

  // start server
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8964").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

async fn path_extract(Path(id): Path<u32>) -> Html<String> {
  Html(format!("<h1>Book ID: {id}</h1>"))
}
