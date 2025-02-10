use super::ApiError;
use crate::db::Counter;
use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn list(State(pool): State<Pool<Sqlite>>) -> Result<Json<Counter>, ApiError> {
  let counter = Counter::get().await?;
  Ok(Json(counter))
}
