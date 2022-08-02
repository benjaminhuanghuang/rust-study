fn main() {
    let text:&str = "hello";

    let text:String = String::from("hello");
    let text:String = "hello".to_string();
    let text:String = "hello".to_owned();

    let text:&str = &text[..];  // slice of entire string


    // 
    let mut s = String::from("foo");
    s.push_str("bar");
    s.replace_range(.., "replaced");

    // Concatenate
    let s1: String = String::from("Hello");
    let s2: String = String::from("wrold!");
    let s3: String  = s1 + &s2; // s1 was moved to s3 and not exited, String type must be first
    
    let s1: String = String::from("Hello");
    let s3: String = s1 + "test";
    
    let s1: String = String::from("Hello");
    let s3: String  = format!("{}{}", s1, s2);
    let s3: String  =["first", "sceond"].concat();
    
    let s3: &str = concat!("first", "second");

    // index
    let s1: &str = "😀the😀😀"; //
    let s2: &str = &s1[0..4];  // first smile face

    for b in "😀the😀😀".bytes() {

    } 

    for b in "😀the😀😀".chars() {

    } 
}


pub fn first_line(string: &str) -> &str {
  string.lines().next().unwrap()
} 