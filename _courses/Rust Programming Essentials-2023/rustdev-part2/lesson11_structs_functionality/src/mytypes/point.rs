use chrono::Utc;  

pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {

    pub fn print(&self) {
        println!("In print(), Point is [{},{}]", self.x, self.y);
    }

    pub fn to_string(&self) -> String {
        format!("[{},{}]", self.x, self.y)
    }

    pub fn reset(&mut self) {
        self.log("About to reset point");
        self.x = 0;
        self.y = 0;
    }
    
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.log("About to move point");
        self.x += dx;
        self.y += dy;
    }

    fn log(&self, msg: &str) {
        let ts = Utc::now().format("%T");
        println!("{} {} at {}", msg, self.to_string(), ts);
    }
}