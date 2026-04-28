use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn function1() {
    println!("In function 1");
    for i in 0..100 {
        println!("ping");
    }
    println!("function 1 done");
}

fn function2() {
    println!("In function 2");
    for i in 0..100 {
        println!("pong");
    }
    println!("function 2 done");
}

fn main() {
    let thread1 = thread::spawn(function1);
    let thread2 = thread::spawn(function2);

    //sleep(Duration::from_nanos(1));

    thread1.join().unwrap();
    thread2.join().unwrap();
}