

It's also possible to `mutably` borrow a value in order to change it without taking ownership.

How to create and use a mutable reference
- in functions
```
  struct Bucket {
    liters: u32
  }

  fn add(source: &mut Bucket){
    source.liters -= 10;
  }

  add(&mut bucket);
```
- In first parameter of method
```
  struct CarPool {
    passengers: Vec<String>
  }

  impl CarPool {
    fn pick_up(&mut self, name: String) {
      self.passengers.push(name);
    }
  }

  let mut car_pool = CarPool{
    passengers: vec![]
  }

  car_pool.pickup(String::from("Jake"))
```

## Borrow rule
