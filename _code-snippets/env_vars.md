## Env
```
  use std::env;

  for (key, val) in env::vars() {
      println!("{}: {}", key, val);
  }

  env::set_var(key, "8080");

  env::remove_var(key);

  fn print_env_var(key: &str) {
    // Accessing an env var
    match env::var(key) {
      Ok(val) => println!("{}: {}", key, val),
      Err(e) => println!("Couldn't print env var {}: {}", key, e),
    }
  }


```


- Server config
```
redis_addr = env::var("REDIS_ADDR").unwrap_or_default("localhost:6379".to_string());
```




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