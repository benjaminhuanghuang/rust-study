


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


```

use std::time::{SystemTime,UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
```