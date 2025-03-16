use dotenv::dotenv;

fn main() {
  // read .env config file
  let database_url = env::var("DATABASE_URL").expect("can not find DATABASE_URL in .env");

  // connect to db
  let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
}
