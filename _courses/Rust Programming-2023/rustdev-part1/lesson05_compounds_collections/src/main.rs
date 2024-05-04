use std::collections::HashMap;

fn main() {

    // Uncomment one of these statements, to run a demo...
    
    // demo_arrays();
    // demo_arrays_techniques();
    // demo_tuples();
    // demo_vectors();
    // demo_maps();
}

fn demo_arrays() {

    println!("Using arrays");
    
    // You can create an array using simple literal syntax.
    let a1 = [100, 101, 102];
    println!("a1 length is {}, first element is {}", a1.len(), a1[0]);

    // You can also create a mutable array - you can change items, but you can't change the size.
    let mut a2 = [100, 101, 102];
    a2[0] = 999;
    println!("a2 length is {}, first element is {}", a2.len(), a2[0]);
    
    // You can iterate over the elements in an array.
    println!("Elements in a2:");
    for elem in a2 {
        println!("  {}", elem);
    }
}

fn demo_arrays_techniques() {

    println!("\nArray techniques");

    // You can specify type info and size.
    let a1: [i64; 5];
    a1 = [100, 101, 102, 103, 104];
    println!("a1 is {:?}", a1);

    // You can fill an array with [filler;size] syntax. 
    let mut a2 = [99; 5];
    a2[0] = 58;
    a2[4] = 25;

    // You can display an array all at once, via the debug formatter.
    println!("a2 is {:?}", a2); 
}

fn demo_tuples() {

    println!("\nUsing tuples");

    // A tuple is a fixed-size heterogeneous collection.
    let t1 = (9, "h1", 3.5);
    println!("t1 elements are {}, {}, {}", t1.0, t1.1, t1.2);

    // You can also create a mutable tuple (you have to be consistent with element types).
    let mut t2 = (9, "hi", 3.5);
    t2.0 = 99;
    println!("t2 elements are {}, {}, {}", t2.0, t2.1, t2.2);

    // You can create an empty tuple (handy for functions that return nothing at all).
    let t3 = ();
    println!("t3 is {:?}", t3);

    // You can specify type info.
    let t4: (i32, bool, f64);
    t4 = (58, true, 1.67);
    println!("t4 is {:?}, elements are {} {} {}", t4, t4.0, t4.1, t4.2);
}

fn demo_vectors() {

    println!("\nUsing vectors");
    
    // You can create a vector object using either of the following syntaxes.
    let mut _v1: Vec<i32> = Vec::new();
	let mut _v2 = Vec::<i32>::new();
	
    // You can create a vector object and initialize it via the vec! macro.
    let mut v3 = vec![100, 101, 102];
  
    // Index into a vector (will panic if index is out-of-bounds).
    let item = v3[0];
    println!("Value: {}", item);
    
    // Index into a vector safely, returns an Option<T>. 
    let opt = v3.get(0);
    match opt {
        Some(value) => println!("Value: {}", value),
        None        => println!("No value")
    }

    // Add and remove items in a vector.
    v3.push(103);
    v3.push(104);
    v3.push(105);
    v3.pop();
    v3.insert(0, 99);
    
    // Iterate over items in a vector.
    println!("Items in v3:");
    for item in &v3 {
        println!("  {}", item)
    }

    // Display a vector all at once, via the debug formatter.
    println!("v3 is {:?}", v3);
}

fn demo_maps() {

    println!("\nUsing maps");
    
    // You can create a map object using either of the following syntaxes.
    let mut m: HashMap<String, i32> = HashMap::new();
    let mut _m2 = HashMap::<String, i32>::new();

    // Insert items.
    m.insert(String::from("UK"), 44);
    m.insert(String::from("NO"), 47);
    m.insert(String::from("SG"), 65);
        
    // Insert item, only if key is missing.
    m.entry(String::from("SA")).or_insert(27);

    // Look-up a key (will panic if key is missing).
    let val = m["UK"];
    println!("Value: {}", val);

    // Look up a key safely, returns an Option<V>.
    let opt = m.get("UK");
    match opt {
        Some(value) => println!("Value: {}", value),
        None        => println!("No value")
    }

    // Iterate over entries in a map.
    println!("Entries in m:");
    for entry in &m {
        println!("  {:?}", entry)
    }
    
    // Display a map all at once, via the debug formatter.
    println!("m is {:?}", m);
}