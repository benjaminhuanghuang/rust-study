use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
  pub code: String,
}

#[derive(Serialize)]
pub struct AuthBody {
  access_token: String,
  token_type: String,
}

impl AuthBody {
  pub fn new(access_token: String) -> Self {
    AuthBody {
      access_token,
      token_type: "Bearer".to_string(),
    }
  }
}

pub async fn login(
  State(pool): State<Pool<Sqlite>>,
  Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthBody>, ApiError> {
  let wx_user = wx_login(payload.code).await?;

  let user = sqlx::query_as::<_, User>(" SELECT * FROM users WHERE openid = ?")
    .bind(wx_user.openid)
    .fetch_one(&pool)
    .await?;
  let user = match user {
    Ok(user) => user,
    Err(sqlx::Error::RowNotFound) => {
      // New user
      sqlx::query("INSERT INTO users (openid, session_key) VALUES (?, ?)")
        .bind(wx_user.openid)
        .bind(wx_user.session_key)
        .execute(&pool)
        .await?;
      sqlx::query_as::<_, User>(" SELECT * FROM users WHERE openid = ?")
        .bind(wx_user.openid)
        .fetch_one(&pool)
        .await?;
    }
    Err(e) => return Err(ApiError::from(e)),
  };

  let claims = Claims::new(user.id.to_string());
  let token = encode(
    &Header::default(),
    &claims,
    //The b prefix converts the "secret" string into a byte array (&[u8])., "secret".as_bytes()
    &EncodingKey::from_secret(b"secret"),
  )
  .map_err(|_| ApiError::Auth(AuthError::TokenCreation))?;

  Ok(Json(response = AuthBody::new(token)))
}

//
#[derive(Deserialize)]
pub struct WxLoginPayload {
  code: String,
}
struct WxUser {
  openid: String,
  session_key: String,
}
async fn wx_login(code: String) -> Result<WxUser, ApiError> {
  Ok(WxUser::default())
}
