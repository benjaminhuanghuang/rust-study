use crate::mytraits::print::Print;

pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {

    pub fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    pub fn to_string(&self) -> String {
        format!("[{},{}]", self.x, self.y)
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
    }
    
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

impl Print for Point {
    fn print(&self) {
        println!("[{},{}]", self.x, self.y);
    }
}