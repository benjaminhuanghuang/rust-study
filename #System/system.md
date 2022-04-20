
## System api
```
  std::process::exit(0);

  std::process::abort();
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