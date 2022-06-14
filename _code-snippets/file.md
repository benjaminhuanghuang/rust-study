```
  use std::fs::File;
  use std::io::{self, BufReader, BufWriter, Read, Result};
```

## Read file to string

```
let data = match fs::read_to_string(&args.filename) {
  Ok(v) => v,
  Err(e) => {
    eprintln!("{} failed to read from file '{}': {:?}", "Error:".red().bold(), args.filename, e);
    std::process::exit(1);
}
};
```

## Read with buffer

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
match fs::write(&args.output, &replaced_data) {
  Ok(v) => v,
  Err(e) => {
    eprintln!("{} failed to write to file '{}': {:?}", "Error:".red().bold(), args.filename, e);
    std::process::exit(1);
  }
};
```

## Write with buffer

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

## rename files in folder

```

fn main() {
    let path = Path::new("./");
    //fs::read_dir() read entities
    for entry in fs::read_dir(path).expect("Path doest not exist.") {
        if let Ok(entry) = entry {
            let file = entry.path();
            let filename = file.to_str().unwrap();
            let new_filename = format!("{}.jpg", filename);
            // rename file
            match fs::rename(filename, &new_filename) {
                Err(why) => panic!("{} => {}: {}", filename, new_filename, why.description()),
                Ok(_) => println!("{} => {}", filename, new_filename),
            }
        }
    }
}
```
