pub enum Color {
    #[allow(dead_code)] Red,
    #[allow(dead_code)] Green,
    #[allow(dead_code)] Blue
}

pub enum HouseLocation {
    #[allow(dead_code)] Number(i32),
    #[allow(dead_code)] Name(String),
    #[allow(dead_code)] Unknown
}