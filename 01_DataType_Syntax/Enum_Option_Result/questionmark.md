# ? operator

```rust

let db = Database::connect();
let db = match db {
    Ok(db) => db,
    Err(err) => {
        return Err(err);
    }
};



let db = Database::connect()?;
```
