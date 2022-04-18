## String lines
```
  for line in quote.lines(){

  }
```
index and line
```
  for (i, line) in quote.lines().enumerate() {

  }
```

## Array
```
  let arrays = [one, two, blank1, blank2];

  for a in &arrays {
  
  }
```

## Vector
```
let nubs = vec![1,2,3,4];


let evens = nums.filter(|x| x%2==0);

let even_squares = events.clone().map(|x| x*x);

let result = event_squares.clone().colect::<Vec<_>>();

```

ref borrow
```
  let mut ctx: Vec<Vec<(usize, String)>> = vec![];

  for item in ctx.iter(){
    for &(i, ref line) in item.iter(){

    }
  }
```

reference & derefrence
```
let needle = 0o204;

let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];
for item in &haystack {
  if *item == needle {
    println!("{}", item);
  }
}
```