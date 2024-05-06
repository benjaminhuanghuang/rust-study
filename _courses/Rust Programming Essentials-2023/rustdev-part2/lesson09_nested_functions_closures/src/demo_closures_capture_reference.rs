pub fn do_it() {
	
    println!("\nIn demo_closures_capture_reference::do_it()");

    capture_immutable_reference();
    capture_mutable_reference();
}

fn capture_immutable_reference() {
    
    let b1 = String::from("┌─────────────────┐");
    let b2 = String::from("└─────────────────┘");

    let display_heading = |s| {
        println!("{}", b1);
        println!("│ {:<15} │", s);
        println!("{}", b2);
    };
    
    display_heading(String::from("hello"));
    display_heading(String::from("goodbye!"));

    println!("b1: {}\nb2: {}\n", b1, b2);
}

fn capture_mutable_reference() {
    
    let mut b1 = String::from("┌─────────────────┐");
    let mut b2 = String::from("└─────────────────┘");

    let mut display_heading = |s| {
        b1.push_str("✅");
        b2.push_str("✅");
        println!("{}", b1);
        println!("│ {:<15} │", s);
        println!("{}\n", b2);
    };
    
    display_heading(String::from("hello"));
    display_heading(String::from("goodbye!"));

    println!("b1: {}\nb2: {}\n", b1, b2);
}