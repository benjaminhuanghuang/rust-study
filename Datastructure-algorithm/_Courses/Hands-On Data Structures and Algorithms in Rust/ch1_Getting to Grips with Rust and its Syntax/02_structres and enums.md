

```
  pub struct Person {
    name: String,
    age: u32
  }

  impl Person {
    pub fn print(self) -> String {
      format!("name = {}, age= {}", self.name, self.age)
    }
  }
```