

{:.*} lets you specify the decimal precision of floating point numbers via the argument
```
format!("{:.*}", 2, 1.234567) // Returns "1.23
```

Name the parameters
```
 let introduction = format!(
    "My name is {surname}, {forename} {surname}", surname="Bond", forename="James"
  );
``` 

pad 0
```
  
```