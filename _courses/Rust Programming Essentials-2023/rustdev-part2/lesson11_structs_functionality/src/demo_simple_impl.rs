#![allow(dead_code)]

struct Point {
    x: i32,
    y: i32
}

impl Point {

    fn print_v1(&self) {
        println!("In print_v1(), point is [{},{}]", self.x, self.y);
    }

    // Alternatively, you can explicitly specify the type of self.
    fn print_v2(self: &Point) {
        println!("In print_v2(), point is [{},{}]", self.x, self.y);
    }

    // Alternatively, you can use Self as an alias for the type of self.
    fn print_v3(self: &Self) {
        println!("In print_v3(), point is [{},{}]", self.x, self.y);
    }

    fn to_string(&self) -> String {
        format!("[{},{}]", self.x, self.y)
    }

}

pub fn do_it() {

    println!("\nIn demo_simple_impl::do_it()");

    let p1 = Point {x: 10, y: 20};
    
    p1.print_v1();
    p1.print_v2();
    p1.print_v3();
    
    println!("{}", p1.to_string());
}