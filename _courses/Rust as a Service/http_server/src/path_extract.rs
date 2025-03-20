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
  println!("Listening on 127.0.0.1:8964");
  axum::serve(listener, app).await.unwrap();
}

/*
http://localhost:8964/book/1
Axum uses extractors to retrieve parts of the request.
Path<u32> means that Axum will parse the path parameter as a u32 number.
The Path(id) syntax destructures the extracted value into the variable id.
*/
async fn path_extract(Path(id): Path<u32>) -> Html<String> {
  Html(format!("<h1>Book ID: {id}</h1>"))
}
