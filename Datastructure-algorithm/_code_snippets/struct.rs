
#[#[derive(Debug)]]
pub struct Person {
  name: String,
  age: i32
}


fn main(){
  let p= Person {
    name: "Ben".to_string();
    age: 100
  } 


  println!("This is {:?}",p );
}