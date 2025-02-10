use axum::{
  http::StatusCode,
  routing::{get, post},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::info;

//
mod db;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  // initialize tracing
  tracing_subscriber::fmt::init();

  let pool = db::establish_connection().await;

  // build our application with a route
  let app = Router::new()
    .route("/api/wx_counter/login", post(api::user::login))
    .layer(TraceLayer::new_for_http())
    .with_state(pool);

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
