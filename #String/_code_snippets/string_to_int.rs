fun test(){
  if let Ok(length) = fields[1].parse::<f32>() {    
    println!("{}, {}cm", name, length);           
  }
}