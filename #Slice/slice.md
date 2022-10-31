https://rust-book.junmajinlong.com/ch3/08_slice.html

```
fn main() {
    // an array of numbers
    let numbers = [1, 2, 3, 4, 5];
    
    // create a slice of 2nd and 3rd element
    let slice = &numbers[1..3];
    
    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);
}
```
&numbers - specifies a reference to the variable numbers (not the actual value)
[1..3] - is a notation for slicing the array from start_index 1 (inclusive) to end_index 3 (exclusive)

Note: A slice is not the actual data like integers or floats but a reference/pointer to the data block. 
That's why we have used the & symbol before the variable name.
