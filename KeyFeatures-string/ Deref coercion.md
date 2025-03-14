# Deref coercion

Rust automatically converts &String to &str because String implements Deref<Target = str>.

Deref coercion allows &String to be treated as &str when passing it to a function expecting &str.

If you wanted to do it explicitly, you could write &word[..] or word.as_str(), which returns a string slice(&str), but the automatic conversion makes this unnecessary.

```rs
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // Don't change this line.

    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

```
