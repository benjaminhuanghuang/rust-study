pub fn do_it() {
	
    println!("\nIn demo_closures_iteration::do_it()");

    demo_simple_iteration();
    demo_unused_closure_variable();
    demo_filtering_maping();
    demo_collecting_result();
}

fn demo_simple_iteration() {

    let v = vec!["donald", "huey", "louie", "dewey"];

    println!("All ducks:");
    v.iter()
     .for_each(|e| println!("   {}", e));
}

fn demo_unused_closure_variable() {

    let v = vec!["donald", "huey", "louie", "dewey"];

    println!("Redacted ducks:");
    v.iter()
     .for_each(|_| println!("   xxx"));
}

fn demo_filtering_maping() {

    let v = vec!["donald", "huey", "louie", "dewey"];

    println!("Uppercase 'd' ducks:");
    v.iter()
      .filter(|e| e.starts_with('d'))
      .map(|e| e.to_uppercase())
      .for_each(|e| println!("   {}", e));
}

fn demo_collecting_result() {

    let v = vec!["donald", "huey", "louie", "dewey"];

    let upper_y_ducks = 
      v.iter()
       .filter(|e| e.ends_with('y'))
       .map(|e| e.to_uppercase())
       .collect::<Vec<String>>();
    
    println!("There are {} ducks ending with 'y':", upper_y_ducks.len());  
    upper_y_ducks.iter()
      .for_each(|e| println!("   {}", e));
}