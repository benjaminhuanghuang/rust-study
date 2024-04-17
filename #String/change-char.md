


```rust

fn set_board(row: &str, col: usize, c: char) -> String {
    let mut chars: Vec<char> = row.chars().collect();
    chars[col] = c;
    chars.into_iter().collect()
}
```
