

## Panic
```
  let result = fs::read_to_string("a.text").unwrap()
```


## Panic with custom error message
```
  let result = fs::read_to_string("a.text").expect("error message");
```

## Matching result<T, E>
```
  let result = fs::read_to_string("a.text");

  let contents = match result {
    Ok(message) => message,
    Err(error) => String::from("Can not find file.")
  }

  println!("contents is {:?}", contents);
```

## std::io::ErrorKind::XXXX
```
  using std::io;

  let result = fs::read_to_string("a.text");

  let contents = match result {
    Ok(message) => message,
    Err(error) => match error.kind() {
      io::ErrorKind::NotFound => String::from("Can not find file."),
      io::ErrorKind::PermissionDenied => String::from("Permission denied."),
      _ => panic!("Another type of error: {:?}", error)
    }
  }

  println!("contents is {:?}", contents);
```

## throw errors (propagating errors)
```
  fn read_and_combine(f1: &str, f2: &str) ->Result<String, io::Error> {
    let mut s1 = match fs::read_to_string(f1) {
      Ok(s) => s,
      Err(e) => return Err(e)
    };
    let mut s2 = match fs::read_to_string(f2) {
      Ok(s) => s,
      Err(e) => return Err(e)
    };
    s1.push('\n');
    s1.push_str(&s2);

    Ok(s1)
  }


  let result = read_and_combine("a.txt", "b.txt");
  match result {
    Ok(s) => println!("result is \n{}", s),
    Err(e) => println!("There was an error:{}", e)
  }
```
Use ? to do the same thing.
```
  fn read_and_combine(f1: &str, f2: &str) ->Result<String, io::Error> {
    let mut s1 = s::read_to_string(f1)?;
    let mut s2 = fs::read_to_string(f2)?;
    s1.push('\n');
    s1.push_str(&s2);

    Ok(s1)
  }

  let result = read_and_combine("a.txt", "b.txt");
  match result {
    Ok(s) => println!("result is \n{}", s),
    Err(e) => println!("There was an error:{}", e)
  }
```


