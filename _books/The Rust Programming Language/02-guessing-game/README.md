

String is a growable, UTF-8 encoded bit of text.
 
An `associated function` is a function that’s implemented on a type, in this case String

```
  let mut guess = String::new();
```


The Result types are enumerations/enums, which can have a fixed set of possibilities known as `variants`. 

Result’s variants are Ok or Err.

If this instance of io::Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect. 

If this instance of io::Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so you can use it.
```
  io::stdin().read_line(&mut guess).expect("Failed to read line");
```



