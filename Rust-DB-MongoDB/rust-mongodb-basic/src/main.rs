use mongodb::bson::doc;
use mongodb::options::ClientOptions;
/*
  Procedural macro in Rust used to define the entry point for asynchronous programs that utilize the Tokio runtime
*/
#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
  let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
  let client = mongodb::Client::with_options(client_options)?;
  let db = client.database("myStudyDB");
  let collection = db.collection("users");
  let result = collection
    .insert_one(doc! { "name": "John Doe", "age":60 })
    .await?;

  println!("Inserted Document id {:#?}", result.inserted_id);
  let mut cursor = collection.find(doc! {}).await?;

  while let Ok(result) = cursor.advance().await {
    match result {
      true => match cursor.deserialize_current() {
        Ok(doc) => println!("Found Document {:#?}", doc),
        Err(e) => eprint!("{:?}", e),
      },
      false => {
        println!("No more documents found");
        break;
      }
    }
  }
  db.drop().await?;

  Ok(())
}
