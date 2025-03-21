use async_trait::async_trait;
use uuid::Uuid;

use crate::{db::DBClient, dtos::TrackDto};

#[async_trait]
pub trait FavoritesExt {
    async fn save_favorite(&self, track_id: Uuid, user_id:: Uuid) -> Result<(), sqlx::Error>;
    async fn delete_favorite(&self, track_id: Uuid, user_id:: Uuid) -> Result<(), sqlx::Error>;
    async fn get_user_favorite_tracks(&self, user_id: Uuid) -> Result<Vec<TrackDto>, sqlx::Error>;
}

impl FavoritesExt for DBClient {
  async fn save_favorite(&self, track_id: Uuid, user_id:: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO favorites (track_id, user_id)
        VALUES ($1, $2)
        "#,
        track_id,
        user_id
    )
    .execute(&self.pool)
    .await?;
    Ok(())
  }
}