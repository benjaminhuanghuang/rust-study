use crate::{db_context::get_database, model::Product};
use surrealdb::error::Db::Thrown;
use surrealdb::Error;
use surrealdb::{engine::remote::ws::Client, Surreal}; // Add this line to import the Error type

pub struct ProductRepository {
  table: String,
  db: Surreal<Client>,
}

impl ProductRepository {
  pub async fn new() -> Self {
    let db = get_database().unwrap();

    ProductRepository {
      table: "product".to_string(),
      db,
    }
  }

  pub async fn get_all(&self) -> Result<Vec<Product>, Error> {
    let products: Vec<Product> = self.db.select(&self.table).await?;

    Ok(products)
  }

  pub async fn get_by_id(&self, id: &str) -> Result<Product, Error> {
    if let Some(record) = self.db.select((&self.table, id)).await? {
      return Ok(record);
    }

    let error = Error::Db(Thrown(format!("Product with id {} not found", id)));

    Err(error)
  }

  pub async fn created_product(&self, product: Product) -> Result<Vec<Product>, Error> {
    let record = self.db.create(&self.table).content(product).await?.unwrap();

    Ok(record)
  }

  pub async fn update_product(&self, id: &str, product: Product) -> Result<Product, Error> {
    let record = self
      .db
      .update((&self.table, id))
      .content(product)
      .await?
      .unwrap();

    Ok(record)
  }

  pub async fn delete_product(&self, id: &str) -> Result<Product, Error> {
    let record = self.db.delete((&self.table, id)).await?.unwrap();

    Ok(record)
  }
}
