```
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::create("D:\\text.txt").unwrap();
    file.write(b"FROM RUST PROGRAM").unwrap();
}

```


```
use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
   
    let mut file = OpenOptions::new()
            .read(true).write(true).open("D:\\text.txt")?;

    file.write(b"COVER")?;

    Ok(())
}
```