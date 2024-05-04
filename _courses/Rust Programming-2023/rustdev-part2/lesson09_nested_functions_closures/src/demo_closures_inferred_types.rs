use chrono::Utc;              
use std::time::Duration;
use std::thread::sleep;

pub fn do_it() {
	
    println!("\nIn demo_closures_inferred_types::do_it()");

    closure_no_params();
    closure_one_param();
    closure_many_params();
    closure_multiple_statements();
}

fn closure_no_params() {
    let get_timestamp = || Utc::now();
    println!("Timestamp:   {}", get_timestamp().format("%T"));
}

fn closure_one_param() {

    let reciprocal = |n| if n == 0.0 {0.0} else {1.0 / n};

    // This statement binds the reciprocal() parameter to floating-point.
    println!("Reciprocal:  {}", reciprocal(5.0));

    // So we can't call reciprocal() with any other type thereafter.
    // println!("Reciprocal:  {}", reciprocal(5));
}

fn closure_many_params() {   
    let prod = |a, b| a * b;
    println!("Product:     {}", prod(20, 5));
}

fn closure_multiple_statements() {
    let get_timestamp_after_delay = |seconds| {
        sleep(Duration::new(seconds, 0));
        Utc::now()
    };   
    println!("Timestamp:   {}", get_timestamp_after_delay(5).format("%T"));
}