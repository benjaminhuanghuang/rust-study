
```
  let num_read = match reader.read(&mut buffer) {
      Ok(x) => x,
      Err(_)
  }*/   
```

```
  let num_read = reader.read(&mut buffer)?;  
```