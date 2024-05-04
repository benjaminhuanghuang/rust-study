#![allow(unused_variables)]        

struct PointV1 {
    x: i32,
    y: i32
}

#[derive(PartialEq)]
struct PointV2 {
    x: i32,
    y: i32
}

pub fn do_it() {

    println!("\nIn demo_partialequality_derived::do_it()");

    demo_no_partial_equality();
    demo_partial_equality();
}

fn demo_no_partial_equality() {

    let p1 = PointV1 {x: 10, y: 20};
    let p2 = PointV1 {x: 10, y: 20};
    let p3 = PointV1 {x: 30, y: 40};

    // These statements won't work.
    // println!("p1 == p2   {}", p1 == p2);
    // println!("p1.eq(&p2) {}", p1.eq(&p2));
    // println!("p1 != p3   {}", p1 != p3);
    // println!("p1.ne(&p3) {}", p1.ne(&p3));
}

fn demo_partial_equality() {

    let p1 = PointV2 {x: 10, y: 20};
    let p2 = PointV2 {x: 10, y: 20};
    let p3 = PointV2 {x: 30, y: 40};

    // These statements will work.
    println!("p1 == p2   {}", p1 == p2);
    println!("p1.eq(&p2) {}", p1.eq(&p2));
    println!("p1 != p3   {}", p1 != p3);
    println!("p1.ne(&p3) {}", p1.ne(&p3));
}