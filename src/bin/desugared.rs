use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

fn main() {
    let number: Arc<Mutex<u64>> = Arc::new(Mutex::new(0u64));
    let alias: Arc<Mutex<u64>> = number.clone();
    thread::spawn(|| {
        let moved: Arc<Mutex<u64>> = alias;
        let mutex: &Mutex<u64> = moved.deref();
        add_loop(mutex);
    });
    loop {
        let mutex: &Mutex<u64> = number.deref();
        let guard: MutexGuard<u64> = mutex.lock().unwrap();
        let number_ref: &u64 = guard.deref();
        println!("{}", *number_ref);
        drop(guard);
        thread::sleep(Duration::from_secs(1));
    }
}

fn add_loop(number: &Mutex<u64>) {
    loop {
        let mut guard: MutexGuard<u64> = number.lock().unwrap();
        let number_mut: &mut u64 = guard.deref_mut();
        *number_mut += 1;
        drop(guard);
    }
}
