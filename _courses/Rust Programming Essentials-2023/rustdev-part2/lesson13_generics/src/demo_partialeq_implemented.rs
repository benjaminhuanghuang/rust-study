struct Angle {
    degrees: i32
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        self.degrees % 360 == other.degrees % 360
    }
}

pub fn do_it() {

    println!("\nIn demo_partialeq_implemented::do_it()");

    let a1 = Angle {degrees: 90};
    let a2 = Angle {degrees: 450};
    let a3 = Angle {degrees: 180};

    println!("a1 == a2   {}", a1 == a2);
    println!("a1.eq(&a2) {}", a1.eq(&a2));
    println!("a1 != a3   {}", a1 != a3);
    println!("a1.ne(&a3) {}", a1.ne(&a3));
}