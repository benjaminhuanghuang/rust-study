#![allow(dead_code)]

struct Point {
    x: i32,
    y: i32
}

impl Point {

    fn print(&self) {
        println!("In print(), Point is [{},{}]", self.x, self.y);
    }

    fn to_string(&self) -> String {
        format!("[{},{}]", self.x, self.y)
    }

    fn reset_v1(&mut self) {
        self.x = 0;
        self.y = 0;
    }
    
    // Alternatively, you can explicitly specify the type of self.
    fn reset_v2(self: &mut Point) {
        self.x = 0;
        self.y = 0;
    }

    // Alternatively, you can use Self as an alias for the type of self.
    fn reset_v3(self: &mut Self) {
        self.x = 0;
        self.y = 0;
    }

    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

pub fn do_it() {

    println!("\nIn demo_mutable_impl::do_it()");

    let mut p1 = Point {x: 10, y: 20};

    p1.move_by(100, 200);
    println!("{}", p1.to_string());

    p1.reset_v1();
    p1.reset_v2();
    p1.reset_v3();
    println!("{}", p1.to_string());
}