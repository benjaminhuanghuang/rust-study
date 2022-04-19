

## string in struct
```
impl TodoMac {
  const TABLE: &'static str = "todo";
  const COLUMNS: &'static [&'static str] = &["id", "cid", "title", "status"];
}
```