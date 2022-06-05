## Array
```
  let list = [1,2,3];

  for item in &list {
    if *item == 8 {

    }
  }
```


```
  let list = [1,2,3];

  for item in list.iter() {
    if *item == 8 {

    }
  }
```

## Vector
Create
```
  let v: Vec<i8> = Vec::new();
```

```
  let list: Vec<i32> = vec![1,2,3];

  for item in list.iter() {
    if *item == 8 {

    }
  }
```


```
  let list: Vec<i32> = vec![1,2,3];

  for item in list.into_iter() {
    if item == 8 {

    }
  }
```
- Change
```
  let mut list: Vec<i32> = vec![1,2,3];

  for item in list.iter_mut() {
    if *item == 8 {
      *item = 10;
    }
  }

  print("{:?}", list);
```
- Range to vec
```
  let bucket = range[count..count + bucket_size].to_vec();
```

## Iterator
```
  struct Stepper{
    cur: i32
    step: i32
    max: i32
  }


  impl Iterrator for Stepper{
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
      if(self.cur >= self.max) {
        return None
      }

      let res:i32 = self.cur;
      self.cur += self.step;
      Some(res)
    }
  }

  let mut ex = Stepper{
    cur:2,
    step:3,
    max:100
  };

  for item in ex {
    println!("{}", item);
  }
```

## nieghbour cells
```
   let _nei: Vec<Position> = (x - 1..=x + 1)
      .flat_map(|i| (y - 1..=y + 1).map(move |j| (i, j)))
      .collect();
```