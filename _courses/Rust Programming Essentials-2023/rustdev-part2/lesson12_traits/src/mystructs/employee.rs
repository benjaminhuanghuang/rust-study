use crate::mytraits::log::Log;
use crate::mytraits::print::Print;

pub struct Employee {
    name: String,
    salary: u64,
    fulltime: bool
}

impl Employee {

    pub fn payrise(&mut self, amount: u64) {
        self.salary += amount
    }

    pub fn new(name: String, salary: u64, fulltime: bool) -> Employee {
        Employee {name, salary, fulltime}
    }
}

impl Print for Employee {
    fn print(&self) {
        println!("{} earns {}, fulltime: {}", self.name, self.salary, self.fulltime);
    }
}

impl Log for Employee {
    const LOG_TIMESTAMP: bool = true;
        
    fn log(&self) {
        println!("{}, {}, {}", self.name, self.salary, self.fulltime);
    }
}