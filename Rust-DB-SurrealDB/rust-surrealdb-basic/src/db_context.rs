use surrealdb::{
  engine::remote::ws::{Client, Ws},
  Result, Surreal,
};

#[tokio::main]
pub async fn get_database() -> Result<Surreal<Client>> {
  let db = Surreal::new::<Ws>("localhost:8000").await?;

  let _ = db.use_ns("product").use_db("product").await?;

  Ok(db)
}
