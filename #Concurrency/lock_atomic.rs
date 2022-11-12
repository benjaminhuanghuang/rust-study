struct Lock<T>{
  locked: AtomicBool,
  data: UnsafeCell<T>
}


impl<T> Lock<T> {
  pub fun new(data: T) -> Lock<T>{
    todo!()
  }

  pub fn lock<R>(&mut self, op:impl FnOnce(&mut T)->R) -> R{
    //spin if we can not get lock
    while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_error() {}

    // execute the operation
    let ret = op(unsafe{&mut *self.v.get()});

    // unlock
    self.locked = false;

  }
}


let l = Lock::new(0);
l.lock(|v| v=+1)