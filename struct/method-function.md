## Method (for instance)
```
  struct Shuttle {
    name: String,
    size: u8,
    propellant: f64
  }

  impl Shuttle {
    fn get_name(&self) -> &str {
      &(self.name);
    }

    fn add_fuel(&mut self, gallons:f64){
      self.propellant += gallones;
    }
  }
```

## Associated Function (associate with type)
```
  impl Shuttle {
    fun new(name: &str) -> Shuttle {
      Shuttle {
        name: String::from(name);
      }
    }
  }
  let mut vehicle = Shuttle::new("Discover");
```
