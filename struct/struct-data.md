```
  #[derive(Debug)]           // implement Debug interface
  struct Shuttle {
    name: String,
    size: u8,
    propellant: f64
  }


  let vehicle = Shuttle {
    name: String::from("ABC"),
    size: 7,
    propellant : 1000.0
  };

  println!("name is {}", vehicle.name);


  let mut vehicle2 = Shuttle {
    name: String::from("123"),
    size: 7,
    propellant : 1000.0
  };

  vehicle2.name = String.from("321")

  println!("vehicle is {:?}", vehicle);


  let mut vehicle3 = Shuttle {
    name: String::from("Discovery"),
    ...vehicle
  };  


  let mut vehicle4 = Shuttle {
    ...vehicle                      // Will own the name of vehicle. can NOT access vehicle.name more
  };

  let mut vehicle4 = Shuttle {
    ...vehicle.clone()              
  };
```