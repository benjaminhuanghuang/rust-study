





```
  for(idx, line) in content.lines().enumerate(){
    if line.contains(search_item) {
      
    }
  }
```


```
  fn load_file(file_name: &str) -> Option<String> {
    let default_path = format!("{}/public",  env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let full_path = format!("{}/{}", public_path, file_name);
    
    let contents =fs::read_to_string(full_path);

    contents.ok()
  }

```