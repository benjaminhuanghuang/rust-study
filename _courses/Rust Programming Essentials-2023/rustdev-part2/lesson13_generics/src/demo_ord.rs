#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct ExamMark {
    value: i32
}

pub fn do_it() {

    println!("\nIn demo_ord::do_it()");

    let m1 = ExamMark {value: 90};
    let m2 = ExamMark {value: 99};
    let m3 = ExamMark {value: 180};
    let m4 = ExamMark {value: -42};

    println!("m1.min(m2) {:?}", m1.min(m2));
    println!("m1.max(m2) {:?}", m1.max(m2));
    println!("m3 clamped {:?}", m3.clamp(ExamMark{value:0}, ExamMark{value:100}));
    println!("m4 clamped {:?}", m4.clamp(ExamMark{value:0}, ExamMark{value:100}));
}