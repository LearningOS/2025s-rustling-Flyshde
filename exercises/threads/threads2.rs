// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::ops::Deref;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // let status = Arc::new();
    let mut handles = vec![];
    let mutex = Arc::new(std::sync::Mutex::new(JobStatus { jobs_completed: 0 }));
    for _ in 0..10 {
        // let mut value = mutex.lock();
        let mutex = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            let mut upadate_status = mutex.lock().unwrap();
            upadate_status.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        let upadate_status = mutex.lock().unwrap();
        println!("jobs completed {}", upadate_status.jobs_completed);
    }
}
