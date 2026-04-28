use std::sync::Arc;
use std::thread;
use std::time::Instant;

const THREADS: usize = 1;

fn sum_part(v: &Vec<u64>, start: usize, end: usize) -> u64 {
    let mut sum = 0;
    for i in start..end {
        sum += v[i];
    }
    return sum;
}

fn main() {
    let mut v: Vec<u64> = Vec::with_capacity(250_000_000);
    for i in 0..250_000_000 {
        v.push(i % 100);
    }
    let v = Arc::new(v);

    let timer = Instant::now();
    let mut threads = Vec::with_capacity(THREADS);
    for i in 0..THREADS {
        let batch_size = v.len() / THREADS;
        let start = batch_size * i;
        let end = start + batch_size;
        let tmp = v.clone();
        threads.push(thread::spawn(move || {
            sum_part(tmp.as_ref(), start, end)
        }));
    }

    let mut total_sum = 0;
    for thread in threads {
        let output = thread.join().unwrap();
        total_sum += output;
    }

    println!("Took {:?}", timer.elapsed());
    println!("Total: {}", total_sum);
}