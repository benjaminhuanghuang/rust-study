use std::fmt::{Debug, Formatter, Result};

// Can implement Copy and Clone implicitly (if all fields implement Copy and Clone).
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Currency {
    pub dollars: i32,
    pub cents: i32
}

impl Currency {
    pub fn new(dollars: i32, cents: i32) -> Currency {
        Currency {dollars, cents}
    }
}

// Alternatively, can implement Copy (and Clone) explicitly.
/*impl Copy for Currency {
}

impl Clone for Currency {

    fn clone(&self) -> Currency {
        println!("Custom cloning of Currency {}.{}", self.dollars, self.cents);
        *self
    }
}
*/