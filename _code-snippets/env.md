
```
[dependencies]

dotenv = "0.15.10"
```

```
  use std::env;

  let default_path = format!("{}/public",  env!("CARGO_MANIFEST_DIR"));

  let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
  
  let full_path = format!("{}/{}", public_path, file_name);
```

```
  use dotenv::dotenv

  dotenv().ok();

  // read .env config file
  let database_url = env::var("DATABASE_URL").expect("can not find DATABASE_URL in .env");

  // connect to db
  let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

```