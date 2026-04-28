use std::ops::{Deref, DerefMut};
use std::thread;

#[derive(Clone, Copy)]
pub struct MyHeapPtr<T> {
    value: *mut T,
}
impl<T> MyHeapPtr<T> {
    pub fn new(t: T) -> MyHeapPtr<T> {
        let b = Box::new(t);
        MyHeapPtr { value: Box::leak(b) }
    }
}

impl<T> Deref for MyHeapPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.value }
    }
}
impl<T> DerefMut for MyHeapPtr<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.value }
    }
}

unsafe impl<T> Send for MyHeapPtr<T> {}
unsafe impl<T> Sync for MyHeapPtr<T> {}


fn main() {
    // Shared counter between all threads
    let mut counter: MyHeapPtr<i32> = MyHeapPtr::new(0);

    // Spawn 5 threads to count
    let mut tasks = Vec::with_capacity(5);
    for i in 0..5 {
        let task = thread::spawn(move || {
            for _ in 0..1_000_000 {
                *counter = *counter + 1;
            }
        });
        tasks.push(task);
    }

    // Wait for all threads to complete
    for task in tasks {
        task.join().unwrap();
    }

    println!("{}", *counter);
}