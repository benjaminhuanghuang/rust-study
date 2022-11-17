fn test() {
  let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

  if cfg!(debug_assertions) {
    eprintln!("debug: {:?} -> {:?}", record, fields);
  }
}


let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);


let (hello, world) = "helloworld".split_at(5);
println!("{}, {}!", hello, world);