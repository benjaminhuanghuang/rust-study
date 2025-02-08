use std::collections::HashMap;
use firebase_rs:: Firebase;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Product {
  name: String,
  description: String,
  price: f32,
  quantity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
  name: String,
}

async fn create_product(firebase_client: &Firebase, product: &Product) -> Response {
  let firebase = firebase_client.at("products");
  let product_res = firebase.set::<Product>(&product).await;

  string_to_response(&product_res.unwrap().data)
}


async fn get_products(firebase_client: &Firebase) -> HashMap<String, Product> {
  let firebase = firebase_client.at("products");
  let products = firebase.get::<HashMap<String, Product>>().await.unwrap();

  products
}


async fn get_product(firebase_client: &Firebase, id: &String) -> Product {
  let firebase = firebase_client.at("products").at(&id);
  let product = firebase.get::<Product>().await;

  return product.unwrap();
}

async fn update_product(firebase_client: &Firebase, id: &String, product: &Product) -> Product {
  let firebase = firebase_client.at("products").at(&id);
  let product = firebase.update::<Product>(&product).await;

  string_to_product(&product.unwrap().data)
}

async fn delete_product(firebase_client: &Firebase, id: &String) {
  let firebase = firebase_client.at("products").at(&id);
  let _result = firebase.delete().await;
}

fn string_to_response(s: &str) -> Response {
  serde_json::from_str(s).unwrap()
}

fn string_to_product(s: &str) -> Product {
  serde_json::from_str(s).unwrap()
}

#[tokio::main]
async fn main() {
  let firebase = Firebase::new("https://ben-firebase-study.firebaseio.com/").unwrap();
  let product = Product {
    name: "test".to_string(),
    description: "test".to_string(),
    price: 100.0,
    quantity: 10,
  };
  let result = create_product(&firebase, &product).await;

  let products = get_products(&firebase).await;
  println!("Products returned {:?}", products);
}
