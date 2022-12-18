
The main idea behind lifetime is ot prevent dangling references
```
  let r;
  {
    let x = 5;
    r = &x;      // borrow value does not live long enough
  } // x is dropped here     

  println("{}", r);
```


## What is a concrete lifetime



