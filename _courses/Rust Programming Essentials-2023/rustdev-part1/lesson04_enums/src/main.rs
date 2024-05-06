// Uncomment either the following 2 statements...
mod mytypes;
use mytypes::{Color, HouseLocation};

// Or uncomment the following 2 statements instead...
// mod mytypes_for_demo_purposes;
// use mytypes_for_demo_purposes::{Color, HouseLocation};

fn main() {

    // Uncomment one of these statements, to run a demo...
    
    // demo_simple_enums();
    // demo_enum_with_data();
    // demo_using_option_enum();
    // demo_using_result_enum();
}

fn demo_simple_enums() {

    println!("Demo simple enums");

    let c: Color = Color::Red;   
    match c {
        Color::Red   => println!("coch"),
        Color::Green => println!("gwyrdd"),
        Color::Blue  => println!("glas")
    }
}

fn demo_enum_with_data() {

    println!("\nDemo enums with data");

    let h: HouseLocation = HouseLocation::Number(4);
    match h {
        HouseLocation::Number(n) => println!("You live in house number {}", n),
        HouseLocation::Name(s)   => println!("You live in a house named {}", s),
        HouseLocation::Unknown   => println!("Your house location is unknown"),
    }

    let size = std::mem::size_of::<HouseLocation>();
    println!("Btw the size of HouseLocation is {} bytes", size);
}

fn demo_using_option_enum() {
    
    println!("\nDemo using the Option<T> enum");
    
    let sec: Option<u32>;

    // Uncomment one of the following statements.
    sec = sec_of_day(23, 59, 59);
    // sec = sec_of_day(1234, 59, 59);

    match sec {
        Some(s) => println!("Second of day: {}", s),
        None    => println!("Second of day: no value available")
    }

    // You can use unwrap_or() to extract Some value from an Option, or use a fallback value if None.
    println!("Unwrapped sec: {}", sec.unwrap_or(0));
}

fn sec_of_day(h: u32, m: u32, s: u32) -> Option<u32> {
    if h <= 23 && m <= 59 && s <= 59 {
        let secs = h * 3600 + m * 60 + s;
        return Option::Some(secs) 
    } 
    else {
        return Option::None
    }
}

fn demo_using_result_enum() {
    
    println!("\nDemo using the Result<T, E> enum");

    let res: Result<i32, std::num::ParseIntError>;
    
    // Uncomment one of the following statements.
    res = i32::from_str_radix("FF", 16);
    // res = i32::from_str_radix("wibble", 16);

    match res {
        Ok(n)  => println!("Parsed str as i32: {}", n),
        Err(e) => println!("Error occurred: {}", e)
    }

    // You can use unwrap_or() to extract Ok value from a Result, or use a fallback value if Err.
    let res2 = i32::from_str_radix("FF", 16);
    println!("Unwrapped result: {}", res2.unwrap_or(-1));
}