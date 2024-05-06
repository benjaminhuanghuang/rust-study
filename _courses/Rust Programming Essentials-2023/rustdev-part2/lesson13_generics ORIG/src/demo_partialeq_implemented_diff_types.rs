#[derive(PartialEq)]
struct TimeSeconds {
    s: i32
}

#[derive(PartialEq)]
struct TimeMinutes {
    m: i32
}

impl PartialEq<TimeMinutes> for TimeSeconds {
    fn eq(&self, other: &TimeMinutes) -> bool {
        self.s == other.m * 60
    }
}

impl PartialEq<TimeSeconds> for TimeMinutes {
    fn eq(&self, other: &TimeSeconds) -> bool {
        other == self
    }
}

pub fn do_it() {

    println!("\nIn demo_partialeq_implemented_diff_types::do_it()");

    let s1 = TimeSeconds {s: 300};
    let s2 = TimeSeconds {s: 300};
    let s3 = TimeSeconds {s: 1234};

    let m1 = TimeMinutes {m: 5};
    let m2 = TimeMinutes {m: 5};
    let m3 = TimeMinutes {m: 5678};

    println!("\nTimeSeconds == TimeSeconds etc.");
    println!("s1 == s2   {}", s1 == s2);
    println!("s1.eq(&s2) {}", s1.eq(&s2));
    println!("s1 != s3   {}", s1 != s3);
    println!("s1.ne(&s3) {}", s1.ne(&s3));

    println!("\nTimeMinutes == TimeMinutes etc.");
    println!("m1 == m2   {}", m1 == m2);
    println!("m1.eq(&m2) {}", m1.eq(&m2));
    println!("m1 != m3   {}", m1 != m3);
    println!("m1.ne(&m3) {}", m1.ne(&m3));

    println!("\nTimeSeconds == TimeMinutes etc.");
    println!("s1 == m1   {}", s1 == m1);
    println!("s1.eq(&m1) {}", s1.eq(&m1));
    println!("s1 != m3   {}", s1 != m3);
    println!("s1.ne(&m3) {}", s1.ne(&m3));

    println!("\nTimeMinutes == TimeSeconds etc.");
    println!("m1 == s1   {}", m1 == s1);
    println!("m1.eq(&s1) {}", m1.eq(&s1));
    println!("m1 != s3   {}", m1 != s3);
    println!("m1.ne(&s3) {}", m1.ne(&s3));
}