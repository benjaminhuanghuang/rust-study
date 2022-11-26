trait Nice {
  fn nice(&self);

  fn change(&mut self);
  
  fn consume(self);
}


struct Hello;


impl Nice for Hello {
  fn nice(&self){}

  fn change(&mut self){}
  
  fn consume(self){}
}


fn main() {
  let mut h = Hello;

  h.nice();

  h.change();

  h.consume();
}