use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

fn main() {
    let number: Arc<Mutex<u64>> = Arc::new(Mutex::new(0u64));
    let alias: Arc<Mutex<u64>> = Arc::clone(&number);
    thread::spawn(move || loop {
        let mut guard: MutexGuard<u64> = alias.lock().unwrap();
        *guard += 1;
    });
    loop {
        let guard: MutexGuard<u64> = number.lock().unwrap();
        println!("{}", *guard);
        drop(guard);
        thread::sleep(Duration::from_secs(1));
    }
}
