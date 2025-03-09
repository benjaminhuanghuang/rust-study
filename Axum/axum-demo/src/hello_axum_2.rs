use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct HelloParams {
  name: Option<String>,
}

#[tokio::main]
async fn main() {
  let routes_all = Router::new().merge(routes_hello());

  // start server
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8964").await.unwrap();
  axum::serve(listener, routes_all).await.unwrap();
}

fn routes_hello() -> Router {
  Router::new()
    .route("/hello", get(handler_hello))
    .route("/hello2/{name}", get(handler_hello2))
}

// /hello?name=Axum
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
  let name = params.name.as_deref().unwrap_or("Axum 2");
  Html(format!("<h1>Hello, {}!</h1>", name))
}

// /hello/Axum
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
  Html(format!("<h1>Hello, {name}!</h1>"))
}
