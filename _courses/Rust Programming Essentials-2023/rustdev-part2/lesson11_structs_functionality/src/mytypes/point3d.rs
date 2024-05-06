use chrono::Utc;  

pub struct Point3D {
    x: i32,
    y: i32,
    z: i32
}

impl Point3D {

    pub fn print(&self) {
        println!("In print(), Point3D is [{},{},{}]", self.x, self.y, self.z);
    }

    pub fn to_string(&self) -> String {
        format!("[{},{},{}]", self.x, self.y, self.z)
    }

    pub fn reset(&mut self) {
        self.log("About to reset point");
        self.x = 0;
        self.y = 0;
        self.z = 0;
    }
    
    pub fn move_by(&mut self, dx: i32, dy: i32, dz: i32) {
        self.log("About to move point");
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }

    fn log(&self, msg: &str) {
        let ts = Utc::now().format("%T");
        println!("{} {} at {}", msg, self.to_string(), ts);
    }

    pub fn new(x: i32, y: i32, z: i32) -> Point3D {
        Point3D {x, y, z}
    }
}