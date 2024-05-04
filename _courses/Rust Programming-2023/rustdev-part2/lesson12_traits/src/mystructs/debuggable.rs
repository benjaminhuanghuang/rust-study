use std::fmt::{Debug, Formatter, Result};

// Can implement Debug implicitly (automatically-generated implementation).
#[derive(Debug)]
pub struct Coord {
    long: f32,
    lat: f32
}

impl Coord {
    pub fn new(long: f32, lat: f32) -> Coord {
        Coord {long, lat}
    }
}

// Alternatively, can implement Debug explicitly.
/*
impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Coord with custom debug format: [{}, {}]", self.long, self.lat)
    }
}
*/

// Can use various Formatter helper functions to help with formatting.
/*
impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("Coordinate structure")
         .field("Longitude", &self.long) 
         .field("Latitude", &self.lat) 
         .finish()
    }
}
*/