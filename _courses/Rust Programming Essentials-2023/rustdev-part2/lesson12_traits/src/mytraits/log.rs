use chrono::Utc;  

pub trait Log {
    
    const LOG_TIMESTAMP: bool = false;

    fn log(&self);  

    fn log_verbose(&self) {
        println!("--------------------------------------------");
        if Self::LOG_TIMESTAMP {
            print!("{} ", Utc::now().format("%T"));   
        }
        self.log();
        println!("--------------------------------------------");
    }
}