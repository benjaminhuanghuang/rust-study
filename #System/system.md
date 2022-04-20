
## System api
```
  std::process::exit(0);

  std::process::abort();
```

## STDOUT, STDERR
redirect channel 1 (STDOUT) to a file `out` and channel 2 (STDERR) to a file `err`
```
cargo run 1>out 2>err
```

## Command line
Use std::env::args
```
  
  println!("{:?}", std::env::args());
  // Args { inner: ["target/debug/mycmd", "Hello", "world", "-n"] }
```

```
cargo run -- -n Hello world
```
positional arguments: position relative to the name of the program (the first element in the arguments) 
determines their meaning


Use clap
```
[dependencies]
clap = "2.33"
```

```
let matches = App::new("echor")
  .version("0.1.0")
  .author("Ken Youens-Clark <kyclark@gmail.com>")
  .about("Rust echo")
  .arg(
    Arg::with_name("text")
    .value_name("TEXT")
    .help("Input text")
    .required(true)
    .min_values(1),
  )
  .arg(
    Arg::with_name("omit_newline")
    .help("Do not print newline")
    .takes_value(false)
    .short("n"),
  )
  .get_matches();
println!("{:#?}", matches);
```




## Compose

command1 && command2
Only if the first process reports success will the second process run:


## test the bin
```
[dev-dependencies]
assert_cmd = "1"
```


```
use assert_cmd::Command;


#[test]
fn false_not_ok() {
  let mut cmd = Command::cargo_bin("false").unwrap();
  cmd.assert().failure();

  // Test stdout 
  cmd.assert().success().stdout("Hello, world!\n");
}
```