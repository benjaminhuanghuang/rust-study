use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
  jobs_completed: u32,
}

/*
we want all of the threads to complete their work
The spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed


Make sure neither of your threads are holding onto the lock of the mutex
while they are sleeping, since this will prevent the other thread from
being allowed to get the lock. Locks are automatically released when
they go out of scope.

*/
fn main() {
  //let status = Arc::new(JobStatus { jobs_completed: 0 });
  let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
  let mut handles = vec![];
  for _ in 0..10 {
    let status_shared = status.clone();
    let handle = thread::spawn(move || {
      thread::sleep(Duration::from_millis(250));
      // TODO: You must take an action before you update a shared value
      let mut counter = status_shared.lock().unwrap();
      counter.jobs_completed += 1;
    });
    handles.push(handle);
  }
  for handle in handles {
    handle.join().unwrap();
    // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
    // interesting in the output? Do you have to 'join' on all the handles?
    println!("jobs completed {}", status.lock().unwrap().jobs_completed);
  }
}
