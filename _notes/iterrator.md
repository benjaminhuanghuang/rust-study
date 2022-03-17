

Iterator::all function to find if all the values are false.
the closure is a predicate or a test that figures out if an
element is false. The values are references, so I compare
each value to &false, which is a reference to a Boolean value.
```
  if [words, bytes, chars, lines].iter().all(|v| v == &false) {
    ...
  }
 ```   


Negate each Boolean value v using std::ops::Not, which is written using a prefix exclamation point (!)
```
 if [lines, words, bytes, chars].iter().all(|v| !v) {
 
 }
```



```
Iterator::any

Iterator::filter

Iterator::map

Iterator::find

Iterator::position

Iterator::cmp

Iterator::min_by

Iterator::max_by
```