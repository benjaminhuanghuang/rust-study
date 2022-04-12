


```
use chrono::Utc;

Utc::now().naive_utc()
```



```
use std::time::{Duration, Instant};
fn main() {
  let mut count = 0;
  // 1 second
  let time_limit = Duration::new(1,0);
  let start = Instant::now();

  while (Instant::now() - start) < time_limit {
    count += 1;
  }
  println!("{}", count);
  }
```