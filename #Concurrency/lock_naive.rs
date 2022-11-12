struct Lock<T>{
  locked: bool,
  data: T
}


impl<T> Lock<T> {
  pub fun new(data: T) -> Lock<T>{
    todo!()
  }

  pub fn lock<R>(&mut self, op:impl FnOnce(&mut T)->R) -> R{
    //spin if we can not get lock
    while self.locked !=false{}

    //lock 
    self.locked = true;
    // --- can case error
    let ret = op(&mut self.data)
    // unlock
    self.locked = false;
    ret
  }
}


let l = Lock::new(0);
l.lock(|v| v=+1)