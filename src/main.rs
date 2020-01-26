use std::sync::{Mutex, Arc};
use std::thread::JoinHandle;
use std::thread;
use std::time::{Duration, Instant};


pub fn psum(val: &Arc<Mutex<u128>>, num: u64) -> Vec<JoinHandle<Duration>> {
    let mut handles = vec![];
    let end = Instant::now() + Duration::from_millis(20);

    for i in 1..=num {
        let counter = Arc::clone(&val);
        let handle = thread::spawn(move || {
            let dur = end - Instant::now();
            thread::sleep(dur);

            println!("At {}", i);
            let mut num = counter.lock().unwrap();
            println!("Doing {}", i);
            *num += i as u128;

            dur
        });
        handles.push(handle);
    }
    handles
}

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let handles = psum(&counter, 10);


    for (i, handle) in handles.into_iter().enumerate() {
        println!("{}: waited {} ns", i, handle.join().unwrap().as_nanos())
    }

    println!("Result: {}", *counter.lock().unwrap());
}
