use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let number = Arc::new(Mutex::new(0u64));
    let alias = Arc::clone(&number);
    thread::spawn(move || add_loop(&alias));
    loop {
        println!("{}", *number.lock().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
}

fn add_loop(number: &Mutex<u64>) {
    loop {
        *number.lock().unwrap() += 1;
    }
}
