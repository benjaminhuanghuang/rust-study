Rust provides the PartialOrd trait to handle the case of sortable things which do not have a total order. 

However, it doesn't provide a standard sort method for Vec<T> where T: PartialOrd. 

The standard idiom to sort a vector in this case is 
```
your_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::{Less|Equal|Greater}));
```