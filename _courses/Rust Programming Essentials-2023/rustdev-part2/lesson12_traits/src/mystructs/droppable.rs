pub struct Place {
    city: String,
    country: String
}

impl Place {
    pub fn new(city: &str, country: &str) -> Place {
        println!("Creating Place with city: {}, country: {}", city, country);
        Place {
            city: city.to_string(),
            country: country.to_string()
        }
    }
}

impl Drop for Place {

    fn drop(&mut self) {
        println!("Dropping Place with city: {}, country: {}", self.city, self.country);
    }
}