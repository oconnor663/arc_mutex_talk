use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let number = Arc::new(Mutex::new(0u64));
    let alias = Arc::clone(&number);
    thread::spawn(move || loop {
        *alias.lock().unwrap() += 1;
    });
    loop {
        println!("{}", *number.lock().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
}
