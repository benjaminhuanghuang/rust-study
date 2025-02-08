use rust_surrealdb_basic::{model::Product, product_repository::ProductRepository};

#[tokio::main]
async fn main() {
  let repository = ProductRepository::new().await;
  let product = Product {
    name: "test".to_string(),
    price: 100.0,
    id: None,
    description: "description".to_string(),
    quantity: 20,
  };
  let result = repository.created_product(product).await.unwrap();
  println!("{:?}", result);
  let products = repository.get_all().await.unwrap();
}
