use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let number = Arc::new(Mutex::new(0u64));
    let number_alias = number.clone();
    thread::spawn(|| {
        let moved_clone = number_alias;
        let mutex_ref = moved_clone.deref();
        add_loop(mutex_ref);
    });
    loop {
        let mutex_ref = number.deref();
        let guard = mutex_ref.lock().unwrap();
        let number_ref = guard.deref();
        println!("{}", *number_ref);
        drop(guard);
        thread::sleep(Duration::from_secs(1));
    }
}

fn add_loop(number: &Mutex<u64>) {
    loop {
        let mut guard = number.lock().unwrap();
        let number_mut = guard.deref_mut();
        *number_mut += 1;
        drop(guard);
    }
}
