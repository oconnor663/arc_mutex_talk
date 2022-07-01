use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

fn main() {
    let number = Arc::new(Mutex::new(0u64));
    let alias = Arc::clone(&number);
    thread::spawn(move || add_loop(&alias));
    loop {
        let guard: MutexGuard<u64> = number.lock().unwrap();
        println!("{}", *guard);
        drop(guard);
        thread::sleep(Duration::from_secs(1));
    }
}

fn add_loop(number: &Mutex<u64>) {
    loop {
        let mut guard: MutexGuard<u64> = number.lock().unwrap();
        *guard += 1;
    }
}
