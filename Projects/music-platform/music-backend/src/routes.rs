use std::sync::Arc;

use axum::{middleware, Extension, Router};
use tower_http::trace::TraceLayer;

use crate::{
  auth::auth,
  handler::{auth::auth_handler, users::users_handler},
  AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
  let api_route = Router::new()
    .nest("/auth", auth_handler())
    .nest("/users", users_handler().layer(middleware::from_fn(auth)))
    .layer(TraceLayer::new_for_http()) // Adds the TraceLayer middleware, which logs request and response details
    .layer(Extension(app_state)); //Adds the shared app_state to the router as an extension

  Router::new().nest("/api", api_route)
}
