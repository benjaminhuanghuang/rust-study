## Chars in string 
```
  for (index, letter) in "abc".chars().enumerate() {
    println!("#{}. letter in the alphabet: {}", index + 1, letter);
  }

```
## Generate alphabetic
In order to be iterable, the range-type has to implement Step. char doesn't, so you wouldn't be able to use 'A'..'D' as an iterator.
```
   let alphabet: Vec<_> = (b'A' .. b'z' + 1) // Start as u8
        .map(|c| c as char)            // Convert all to chars
        .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
        .collect(); // Collect as Vec<char>

```

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