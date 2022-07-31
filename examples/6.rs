use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static NUMBER: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(rand::random()));

fn main() {
    thread::spawn(|| loop {
        *NUMBER.lock().unwrap() += 1;
    });
    loop {
        println!("{}", *NUMBER.lock().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
}
