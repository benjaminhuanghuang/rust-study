use chrono::{DateTime, Utc};   
use std::time::Duration;
use std::thread::sleep;

pub fn do_it() {
	
    println!("\nIn demo_closures::do_it()");

    closure_no_params();
    closure_one_param();
    closure_many_params();
    closure_multiple_statements();
}

fn closure_no_params() {
    let get_timestamp = || -> DateTime<Utc> { Utc::now() };
    println!("Timestamp:   {}", get_timestamp());//.format("%T"));
}

fn closure_one_param() {
    let reciprocal = |n: f64| -> f64 { if n == 0.0 {0.0} else {1.0 / n} };
    println!("Reciprocal:  {}", reciprocal(5.0));
}

fn closure_many_params() {
    let prod = |a: i32, b: i32| -> i32 { a * b };
    println!("Product:     {}", prod(20, 5));
}

fn closure_multiple_statements() {
    let get_timestamp_after_delay = |seconds: u64| -> DateTime<Utc> {
        sleep(Duration::new(seconds, 0));
        Utc::now()
    };  
    println!("Timestamp:   {}", get_timestamp_after_delay(5).format("%T"));
}