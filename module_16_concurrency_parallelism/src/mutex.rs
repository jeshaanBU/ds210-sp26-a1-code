use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared counter between all threads
    let mut counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    // Spawn 5 threads to count
    let mut tasks = Vec::with_capacity(5);
    for i in 0..5 {
        let clone = counter.clone();
        let task = thread::spawn(move || {
            println!("thread {i} queued");
            println!("thread {i} started");
            for _ in 0..1_000_000 {
                let mut c = clone.lock().unwrap();
                *c = *c + 1;
            }
            println!("thread {i} finished");
        });
        tasks.push(task);
    }

    // Wait for all threads to complete
    for task in tasks {
        task.join().unwrap();
    }

    let c = counter.lock().unwrap();
    println!("{}", c);
}