use std::cmp::Ordering;

#[derive(PartialOrd, PartialEq)]
struct Currency {
    dollars: i32,
    cents: i32
}

struct Angle {
    degrees: i32
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let d1 = self.degrees % 360;
        let d2 = other.degrees % 360;
        Some(d1.cmp(&d2))
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        self.degrees % 360 == other.degrees % 360
    }
}

pub fn do_it() {

    println!("\nIn demo_partialord::do_it()");

    demo_derived();
    demo_implemented();
}

fn demo_derived() {
    
    println!("\nDerived PartialOrd");

    let c1 = Currency {dollars: 10, cents: 75};
    let c2 = Currency {dollars: 20, cents: 50};
    let c3 = Currency {dollars: 20, cents: 75};

    println!("c1 < c2   {}", c1 < c2);
    println!("c1.lt(c2) {}", c1.lt(&c2));
    println!("c1 <= c2  {}", c1 <= c2);
    println!("c1.le(c2) {}", c1.le(&c2));

    println!("c1 > c2   {}", c1 > c2);
    println!("c1.gt(c2) {}", c1.gt(&c2));
    println!("c1 >= c2  {}", c1 >= c2);
    println!("c1.ge(c2) {}", c1.ge(&c2));   

    println!("c2 < c3   {}", c2 < c3);
    println!("c2 > c3   {}", c2 > c3);
}

fn demo_implemented() {
    
    println!("\nImplemented PartialOrd");

    let a1 = Angle {degrees: 10};
    let a2 = Angle {degrees: 400};

    println!("a1 <  a2  {}", a1 < a2);
    println!("a1.lt(a2) {}", a1.lt(&a2));
    println!("a1 <= a2  {}", a1 <= a2);
    println!("a1.le(a2) {}", a1.le(&a2));

    println!("a1 >  a2  {}", a1 > a2);
    println!("a1.gt(a2) {}", a1.gt(&a2));
    println!("a1 >= a2  {}", a1 >= a2);
    println!("a1.ge(a2) {}", a1.ge(&a2));   
}