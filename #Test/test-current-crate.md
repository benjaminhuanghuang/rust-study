# Test the bin in the target/debug directory
```
[dev-dependencies]
assert_cmd = "1"
```

```
use assert_cmd::Command;

#[test]
fn runs() {
  let mut cmd = Command::cargo_bin("hello").unwrap();

  // Assert::success() to “ensure the command succeeded.”
  cmd.assert().success();
}
```