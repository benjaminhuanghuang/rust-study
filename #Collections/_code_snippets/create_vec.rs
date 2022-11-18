// Empty buffer
let mut my_vector = Vec::with_capacity(10);
let mut v = vec![];
let mut v = Vec::new();
let mut vec![0; count]


// Create and init
let mut vector = vec![1,2,3,42,5];
let mut vec = vec![];
for _ in 0..count {
    vec.push(0);
}
    

// from string
let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
let vec: Vec<&str> = contents.split(",").collect();
println!("{:?}",vec);
