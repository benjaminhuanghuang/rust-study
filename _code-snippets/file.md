
```
  use std::fs::File;
  use std::io::{self, BufReader, BufWriter, Read, Result};
```

## Read
```
  // Box is a smart pointer with a fixed size
  let mut reader: Box<dyn Read> = if !infile.is_empty() {
      Box::new(BufReader::new(File::open(infile)?))
  } else {
      Box::new(BufReader::new(io::stdin()))
  };

```

## Write
```    
 let mut writer: Box<dyn Write> = if !outfile.is_empty() {
    Box::new(BufWriter::new(File::create(outfile)?))
  } else {
    Box::new(BufWriter::new(io::stdout()))
  };

``` 





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